use clap::Parser;
use opencv::{core, highgui, imgcodecs, imgproc, prelude::*, videoio, Result};
use prettytable::{color, Attr, Cell, Row, Table};

use fern::Dispatch;
use log::{debug, info, trace, warn};
use std::{io, time::SystemTime};

fn setup_logging(verbosity: u64) -> Result<(), fern::InitError> {
    let mut base_config = fern::Dispatch::new();

    base_config = match verbosity {
        0 => {
            // Let's say we depend on something which whose "info" level messages are too
            // verbose to include in end-user output. If we don't need them,
            // let's not include them.
            base_config
                .level(log::LevelFilter::Info)
                .level_for("overly-verbose-target", log::LevelFilter::Warn)
        }
        1 => base_config
            .level(log::LevelFilter::Debug)
            .level_for("overly-verbose-target", log::LevelFilter::Info),
        2 => base_config.level(log::LevelFilter::Debug),
        _3_or_more => base_config.level(log::LevelFilter::Trace),
    };

    // Separate file config so we can include year, month and day in file logs
    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(fern::log_file("program.log")?);

    let stdout_config = fern::Dispatch::new()
        .format(|out, message, record| {
            // special format for debug messages coming from our own crate.
            if record.level() > log::LevelFilter::Info && record.target() == "cmd_program" {
                out.finish(format_args!(
                    "DEBUG @ {}: {}",
                    humantime::format_rfc3339_seconds(SystemTime::now()),
                    message
                ))
            } else {
                out.finish(format_args!(
                    "[{} {} {}] {}",
                    humantime::format_rfc3339_seconds(SystemTime::now()),
                    record.level(),
                    record.target(),
                    message
                ))
            }
        })
        .chain(io::stdout());

    base_config
        .chain(file_config)
        .chain(stdout_config)
        .apply()?;

    Ok(())
}

/// Configurações do programa obtidas via linha de comando
#[derive(Parser, Debug)]
#[command(
    author = "Vilante Labs (C) 2025", // Flavio Vilante
    version = env!("CARGO_PKG_VERSION"),
    about = "video_frame_saver",
    long_about = "

video_frame_saver - Ferramenta para captura de vídeo em tempo real.

FUNCIONALIDADES PRINCIPAIS:
- Captura o vídeo da câmera especificada pelo usuário.
- Exibe o vídeo em uma janela, com opção de redimensionamento da imagem.
- Permite salvar um frame da imagem exibida pressionando a tecla ENTER.
- Scaneia portas para identificar cameras acessiveis
    "
)]
struct Config {
    /// Índice da câmera a ser usada
    #[arg(short = 'c', long, default_value_t = 0)]
    camera_index: i32,

    /// Fator de escala horizontal
    #[arg(short = 'x', long, default_value_t = 1.0)]
    scale_x: f64,

    /// Fator de escala vertical
    #[arg(short = 'y', long, default_value_t = 1.0)]
    scale_y: f64,

    /// Nome do arquivo de saída
    #[arg(short = 'f', long, default_value = "imagem_capturada.bmp")]
    image_name: String,

    /// Detecta e apresenta a listagem das câmeras acessíveis
    #[arg(short = 'l', long)]
    list: bool,

    /// Range inicial para buscar câmeras
    #[arg(long, default_value_t = 0)]
    range_start: i32,

    /// Range final para buscar câmeras
    #[arg(long, default_value_t = 10)]
    range_end: i32,

    /// Largura desejada da câmera (opcional). Utilize em conjunto com o parametro 'height'
    #[arg(long)]
    width: Option<i32>,

    /// Altura desejada da câmera (opcional). Utilize em conjunto com o parametro 'width'
    #[arg(long)]
    height: Option<i32>,

    /// Ativa modo verbose para depuração (ativa todas as opções de saída detalhada)
    #[arg(long)]
    verbose: bool,

    /// Detecta e lista as opcoes de resolucao da camera.
    #[arg(long)]
    detect_resolutions: bool,
}

#[derive(Debug)]
enum AppError {
    CameraOpenError(String),
    CameraCloseError(String),
    FrameReadError(String),
    WriteImageError(String),
    CameraSizeError(String),
    CameraGuiWindowOpenError(String),
    ImageResizeError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::CameraOpenError(msg) => write!(f, "Erro ao abrir a camera: {}", msg),
            AppError::CameraCloseError(msg) => write!(f, "Erro ao fechar a camera: {}", msg),
            AppError::FrameReadError(msg) => write!(f, "Erro ao ler frame: {}", msg),
            AppError::WriteImageError(msg) => write!(f, "Erro ao salvar imagem: {}", msg),
            AppError::CameraSizeError(msg) => write!(f, "Erro ao obter tamanho da camera: {}", msg),
            AppError::CameraGuiWindowOpenError(msg) => write!(
                f,
                "Erro ao abrir janela grafica para apresentar imagem da camera: {}",
                msg
            ),
            AppError::ImageResizeError(msg) => {
                write!(f, "Erro ao redimensionar (escalar) imagem: {}", msg)
            }
        }
    }
}

impl std::error::Error for AppError {}

impl AppError {
    /// Retorna codigo de saida especifico para cada tipo de erro
    fn exit_code(&self) -> i32 {
        match self {
            AppError::CameraOpenError(_) => 60,
            AppError::FrameReadError(_) => 61,
            AppError::WriteImageError(_) => 62,
            AppError::CameraSizeError(_) => 63,
            AppError::CameraGuiWindowOpenError(_) => 64,
            AppError::CameraCloseError(_) => 65,
            AppError::ImageResizeError(_) => 66,
        }
    }
}

/// Inicializa a câmera com o índice especificado
fn initialize_camera(camera_index: i32) -> Result<videoio::VideoCapture, AppError> {
    let camera = videoio::VideoCapture::new(camera_index, videoio::CAP_ANY)
        .map_err(|e| AppError::CameraOpenError(e.to_string()))?;
    if !camera
        .is_opened()
        .map_err(|e| AppError::CameraOpenError(e.to_string()))?
    {
        return Err(AppError::CameraOpenError(format!(
            "Não foi possível abrir a câmera com o índice: {}",
            camera_index
        )));
    }
    Ok(camera)
}

/// Libera os recursos associados à câmera
fn release_camera(mut camera: videoio::VideoCapture) -> Result<(), AppError> {
    camera
        .release()
        .map_err(|e| AppError::CameraCloseError(e.to_string()))
}

fn lista_cameras_acessiveis(range: impl Iterator<Item = i32>) -> Vec<i32> {
    range
        .filter_map(|index| {
            if let Ok(camera) = initialize_camera(index) {
                let _ = release_camera(camera);
                Some(index)
            } else {
                None
            }
        })
        .collect()
}

fn imprime_lista_de_cameras_acessiveis(range: impl Iterator<Item = i32> + Clone) {
    let cameras_disponiveis = lista_cameras_acessiveis(range.clone());
    let cameras_nao_disponiveis: Vec<i32> = range
        .filter(|index| !cameras_disponiveis.contains(index))
        .collect();

    let mut table = Table::new();

    // Adiciona o cabeçalho
    table.add_row(Row::new(vec![
        Cell::new("Índice da Câmera")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::BRIGHT_WHITE)),
        Cell::new("Status")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::BRIGHT_WHITE)),
    ]));

    // Adiciona câmeras disponíveis
    for index in cameras_disponiveis {
        table.add_row(Row::new(vec![
            Cell::new(&index.to_string()).with_style(Attr::ForegroundColor(color::BRIGHT_GREEN)),
            Cell::new("Disponível").with_style(Attr::ForegroundColor(color::BRIGHT_GREEN)),
        ]));
    }

    // Adiciona câmeras não disponíveis
    for index in cameras_nao_disponiveis {
        table.add_row(Row::new(vec![
            Cell::new(&index.to_string()).with_style(Attr::ForegroundColor(color::BRIGHT_RED)),
            Cell::new("Não Disponível").with_style(Attr::ForegroundColor(color::BRIGHT_RED)),
        ]));
    }

    // Exibe a tabela
    table.printstd();
}

struct CameraSize {
    width: f64,
    height: f64,
}

fn camera_set_size(camera: &mut videoio::VideoCapture, config: &Config) -> Result<(), AppError> {
    if let (Some(w), Some(h)) = (config.width, config.height) {
        if config.verbose {
            println!("Definindo resolução da câmera para {}x{}", w, h);
        }

        for (prop, val, desc) in [
            (videoio::CAP_PROP_FRAME_WIDTH, w as f64, "largura"),
            (videoio::CAP_PROP_FRAME_HEIGHT, h as f64, "altura"),
        ] {
            camera.set(prop, val).map_err(|e| {
                AppError::CameraSizeError(format!(
                    "Erro ao definir {} da câmera para {} pixels: {}",
                    desc, val, e
                ))
            })?;
        }
    } else if config.verbose {
        println!("Usando resolução padrão da câmera.");
    }
    Ok(())
}

fn camera_get_size(camera: &videoio::VideoCapture) -> Result<CameraSize, AppError> {
    let width = camera
        .get(videoio::CAP_PROP_FRAME_WIDTH)
        .map_err(|e| AppError::CameraSizeError(format!("Erro ao obter largura: {}", e)))?;
    let height = camera
        .get(videoio::CAP_PROP_FRAME_HEIGHT)
        .map_err(|e| AppError::CameraSizeError(format!("Erro ao obter altura: {}", e)))?;
    if width <= 0.0 || height <= 0.0 {
        return Err(AppError::CameraSizeError(format!(
            "Dimensões inválidas: largura e altura devem ser maiores que zero"
        )));
    }
    Ok(CameraSize { width, height })
}

/// Exibe os frames capturados da câmera, redimensionados com os fatores especificados
fn display_camera_feed(
    camera: &mut videoio::VideoCapture,
    config: &Config,
) -> Result<(), AppError> {
    highgui::named_window("window", highgui::WINDOW_FULLSCREEN).map_err(|e| {
        AppError::CameraGuiWindowOpenError(format!(
            "Erro ao abrir janela grafica para apresentar imagem da camera: {}",
            e
        ))
    })?;
    let mut frame = Mat::default();
    let mut frame_resized = Mat::default();

    loop {
        camera
            .read(&mut frame)
            .map_err(|e| AppError::FrameReadError(format!("Erro ao ler frame da camera: {}", e)))?;

        if frame
            .size()
            .map_err(|e| {
                AppError::FrameReadError(format!(
                    "Erro ao ler comprimento do frame da camera: {}",
                    e
                ))
            })?
            .width
            > 0
        {
            imgproc::resize(
                &frame,
                &mut frame_resized,
                core::Size::new(0, 0),
                config.scale_x,
                config.scale_y,
                imgproc::INTER_LINEAR,
            )
            .map_err(|e| {
                AppError::ImageResizeError(format!("Erro redimensionar (escalar) imagem: {}", e))
            })?;
            highgui::imshow("window", &mut frame_resized).map_err(|e| {
                AppError::CameraGuiWindowOpenError(format!(
                    "Erro ao mostrar janela grafica para apresentar imagem da camera: {}",
                    e
                ))
            })?;
        } else {
            return Err(AppError::FrameReadError(format!(
                "Erro comprimento do frame retornou um valor negativo"
            )));
        }

        let key = highgui::wait_key(10).map_err(|e| {
            AppError::FrameReadError(format!("Erro ao ler tecla precionada: {}", e))
        })?;

        if process_key_input(key, &frame, &config.image_name)? {
            break;
        }
    }

    Ok(())
}

/// Processa a entrada do teclado durante a exibição do feed da câmera
fn process_key_input(key: i32, frame: &Mat, image_name: &str) -> Result<bool, AppError> {
    if key > 0 && key != 255 {
        if key == 27 {
            println!("Pressionado tecla ESC, nenhuma imagem será salva.");
            return Ok(true);
        }

        println!("Iniciando rotina de gravação...");
        let params = core::Vector::new();
        match imgcodecs::imwrite(image_name, frame, &params) {
            Ok(_) => {
                println!("Gravação bem-sucedida! Arquivo salvo como: {}", image_name);
                return Ok(true);
            }
            Err(e) => {
                return Err(AppError::WriteImageError(format!(
                    "Erro ao salvar imagem: {}",
                    e
                )))
            }
        }
    }
    Ok(false)
}

fn listar_resolucoes_suportadas(
    camera: &mut videoio::VideoCapture,
) -> Result<Vec<(i32, i32)>, AppError> {
    let resolucoes_comuns = vec![
        (160, 120),
        (320, 240),
        (640, 480),
        (800, 600),
        (1024, 768),
        (1280, 720),
        (1280, 800),
        (1280, 1024),
        (1366, 768),
        (1440, 900),
        (1600, 900),
        (1680, 1050),
        (1920, 1080),
        (1920, 1200),
        (2560, 1440),
        (2560, 1600),
        (3840, 2160),
        (4096, 2160),
        (5120, 2880),
        (7680, 4320),
        (3264, 2448),
        (2592, 1944),
        (2048, 1536),
        (960, 540),
        (720, 576),
        (854, 480),
        (1152, 864),
        (1360, 768),
        (3000, 4000),
        (4000, 3000),
    ];

    let mut resolucoes_suportadas = Vec::new();

    for (w, h) in resolucoes_comuns.iter() {
        camera
            .set(videoio::CAP_PROP_FRAME_WIDTH, *w as f64)
            .map_err(|e| {
                AppError::CameraSizeError(format!(
                    "Erro ao definir largura da camera para {} pixels: {}",
                    w, e
                ))
            })?;
        camera
            .set(videoio::CAP_PROP_FRAME_HEIGHT, *h as f64)
            .map_err(|e| {
                AppError::CameraSizeError(format!(
                    "Erro ao definir altura da camera para {} pixels: {}",
                    w, e
                ))
            })?;

        let width = camera.get(videoio::CAP_PROP_FRAME_WIDTH).map_err(|e| {
            AppError::CameraSizeError(format!("Erro ao obter largura da camera: {}", e))
        })? as i32;
        let height = camera.get(videoio::CAP_PROP_FRAME_HEIGHT).map_err(|e| {
            AppError::CameraSizeError(format!("Erro ao obter altura da camera: {}", e))
        })? as i32;

        if width == *w && height == *h {
            resolucoes_suportadas.push((*w, *h));
        }
    }

    Ok(resolucoes_suportadas)
}

fn run_app() -> Result<(), AppError> {
    let verbosity: u64 = 0; //cmd_arguments.occurrences_of("verbose");
    setup_logging(verbosity).expect("failed to initialize logging.");

    info!("Hello, world!");
    warn!("Warning!");
    debug!("Now exiting.");

    // Parse dos argumentos via linha de comando
    let config = Config::parse();

    // Verifica se o modo de listagem foi solicitado
    if config.list {
        let range = config.range_start..config.range_end;
        imprime_lista_de_cameras_acessiveis(range);
        return Ok(());
    }

    // Inicializa a câmera
    let mut camera = initialize_camera(config.camera_index)?;
    println!("Câmera inicializada com sucesso!");

    if config.detect_resolutions {
        let resolucoes = listar_resolucoes_suportadas(&mut camera)?;
        println!("Resoluções suportadas pela câmera:");
        for (w, h) in resolucoes {
            println!("{}x{}", w, h);
        }
        return Ok(());
    } else {
        // Seta dimensoes da camera
        camera_set_size(&mut camera, &config)?;

        // Obtém e exibe as dimensões da câmera
        let camera_size = camera_get_size(&camera)?;
        println!(
            "Dimensões atuais da câmera: Largura = {}, Altura = {}",
            camera_size.width, camera_size.height
        );

        // Exibe o feed da câmera
        display_camera_feed(&mut camera, &config)?;

        Ok(())
    }
}

fn main() {
    match run_app() {
        Ok(_) => std::process::exit(0), // Sucesso retorna 0
        Err(e) => {
            eprintln!("Erro: {}", e);
            std::process::exit(e.exit_code()); // Retorna o codigo de erro especifico
        }
    }
}
