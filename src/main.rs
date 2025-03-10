use clap::builder::styling::Style;
use clap::builder::Styles;
use clap::ColorChoice;
use clap::CommandFactory;
use clap::Parser;
use opencv::{core, highgui, imgcodecs, imgproc, prelude::*, videoio, Result};
use prettytable::{color, Attr, Cell, Row, Table};
use std::env;

use fern::colors::{Color, ColoredLevelConfig};
use log::{debug, error, info, trace, warn};
use std::{io, time::SystemTime};

fn setup_logging(config: &Config) -> Result<(), fern::InitError> {
    // configure colors for the whole line
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        // we actually don't need to specify the color for debug and info, they are white by default
        .info(Color::White)
        .debug(Color::White)
        // depending on the terminals color scheme, this is the same as the background color
        .trace(Color::BrightBlack);

    // configure colors for the name of the level.
    // since almost all of them are the same as the color for the whole line, we
    // just clone `colors_line` and overwrite our changes
    let colors_level = colors_line.info(Color::Green);

    let mut base_config = fern::Dispatch::new();

    // Se o usuário passou explicitamente --log-file, adicionamos log para arquivo
    if config.log_file {
        let log_file_name = config.log_file_name.clone();
        // Correção aqui
        let file_config = fern::Dispatch::new()
            .level(log::LevelFilter::Trace)
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{} {} {}] {}",
                    humantime::format_rfc3339_seconds(SystemTime::now()),
                    record.level(),
                    record.target(),
                    message
                ))
            })
            .chain(fern::log_file(log_file_name)?); // Usa o nome do arquivo fornecido pelo usuário

        base_config = base_config.chain(file_config);
    }

    // Configuração do log para stdout
    let stdout_config = fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}[{date} {level} {target} {color_line}] {message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    colors_line.get_color(&record.level()).to_fg_str()
                ),
                date = humantime::format_rfc3339_seconds(SystemTime::now()),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message,
            ));
        })
        // change log levels for individual modules. Note: This looks for the record's target
        // field which defaults to the module path but can be overwritten with the `target`
        // parameter:
        // `info!(target="special_target", "This log message is about special_target");`
        .level_for("pretty_colored", log::LevelFilter::Trace)
        // output to stdout
        .chain(std::io::stdout());

    // Aplica a configuração de logging
    base_config.chain(stdout_config).apply()?;

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
#[command(styles=get_styles())]
struct Config {
    /// Índice da câmera a ser usada.
    #[arg(short = 'c', long, default_value_t = 0)]
    camera_index: i32,

    /// Fator de escala horizontal.
    #[arg(short = 'x', long, default_value_t = 1.0)]
    scale_x: f64,

    /// Fator de escala vertical.
    #[arg(short = 'y', long, default_value_t = 1.0)]
    scale_y: f64,

    /// Nome do arquivo de saída.
    #[arg(short = 'f', long, default_value = "imagem_capturada.bmp")]
    image_name: String,

    /// Detecta e apresenta a listagem das câmeras acessíveis.
    #[arg(short = 'l', long)]
    list: bool,

    /// Range inicial para buscar câmeras.
    #[arg(long, default_value_t = 0)]
    range_start: i32,

    /// Range final para buscar câmeras.
    #[arg(long, default_value_t = 10)]
    range_end: i32,

    /// Largura desejada da câmera (opcional). Utilize em conjunto com o parametro 'height'
    #[arg(long)]
    width: Option<i32>,

    /// Altura desejada da câmera (opcional). Utilize em conjunto com o parametro 'width'
    #[arg(long)]
    height: Option<i32>,

    /// Ativa modo verbose para depuração (ativa todas as opções de saída detalhada).
    #[arg(long)]
    verbose: bool,

    /// Detecta e lista as opcoes de resolucao da camera.
    #[arg(long)]
    detect_resolutions: bool,

    /// Ativa gravação de dados de telemetria em arquivo de log. Se o arquivo já existir, concatena dados.
    #[arg(long)]
    log_file: bool,

    /// Escolhe um nome para o arquivo de log. Utilize em conjunto com '--log-file'.
    #[arg(short = 'o', long, default_value = "video_frame_saver.log")]
    log_file_name: String,
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
            "Nao foi possivel abrir a camera com o indice: {}",
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

fn lista_cameras_acessiveis(range: impl Iterator<Item = i32> + Clone) -> Vec<i32> {
    let vec_range: Vec<i32> = range.clone().collect(); // Clona os valores do iterador

    if let (Some(first), Some(last)) = (vec_range.first(), vec_range.last()) {
        trace!("Buscando camera entre IDs {} e {}", first, last);
    }

    vec_range
        .into_iter()
        .filter_map(|index| {
            if let Ok(camera) = initialize_camera(index) {
                trace!("Camera {} acessivel", index);
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
            info!("Definindo resolução da camera para {}x{}", w, h);
        }

        for (prop, val, desc) in [
            (videoio::CAP_PROP_FRAME_WIDTH, w as f64, "largura"),
            (videoio::CAP_PROP_FRAME_HEIGHT, h as f64, "altura"),
        ] {
            camera.set(prop, val).map_err(|e| {
                AppError::CameraSizeError(format!(
                    "Erro ao definir {} da camera para {} pixels: {}",
                    desc, val, e
                ))
            })?;
        }
    } else if config.verbose {
        warn!("Necessario preencher os parametros `--width` e `--height` em conjunto. Como alternativa selecionando resolução padrão da câmera.");
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
            "Dimensoes invalidas: largura e altura devem ser maiores que zero"
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
            info!("Pressionado tecla ESC, nenhuma imagem sera salva.");
            return Ok(true);
        }

        info!("Iniciando rotina de gravacao...");
        let params = core::Vector::new();
        match imgcodecs::imwrite(image_name, frame, &params) {
            Ok(_) => {
                info!("Gravacao bem-sucedida! Arquivo salvo como: {}", image_name);
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

fn run_app(config: &Config) -> Result<(), AppError> {
    // Verifica se o modo de listagem foi solicitado
    if config.list {
        let range = config.range_start..config.range_end;
        imprime_lista_de_cameras_acessiveis(range);
        return Ok(());
    }

    // Inicializa a câmera
    let mut camera = initialize_camera(config.camera_index)?;
    info!("Camera inicializada com sucesso!");

    if config.detect_resolutions {
        let resolucoes = listar_resolucoes_suportadas(&mut camera)?;
        info!("Resolucoes suportadas pela camera:");
        for (w, h) in resolucoes {
            info!("{}x{}", w, h);
        }
        return Ok(());
    } else {
        // Seta dimensoes da camera
        camera_set_size(&mut camera, &config)?;

        // Obtém e exibe as dimensões da câmera
        let camera_size = camera_get_size(&camera)?;
        info!(
            "Dimensoes atuais da camera: Largura = {}, Altura = {}",
            camera_size.width, camera_size.height
        );

        // Exibe o feed da câmera
        display_camera_feed(&mut camera, &config)?;

        Ok(())
    }
}

pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}

enum { EXIT_SUCESS = 0 };

fn main() {
    // Parse dos argumentos via linha de comando
    let config = Config::parse();
    // versao do programa
    let version = env!("CARGO_PKG_VERSION");

    // configura logging
    setup_logging(&config).expect("Falha ao iniciar logging.");
    trace!("Programa iniciado - versao {}", version);
    let full_command: Vec<String> = env::args_os()
        .map(|s| s.to_string_lossy().into_owned())
        .collect();
    trace!("Parametros: {}", full_command.join(" "));

    // executa aplicativo
    match run_app(&config) {
        Ok(_) => {
            trace!("Programa finalizado com sucesso");
            std::process::exit(EXIT_SUCCESS);
        } // Sucesso retorna 0
        Err(e) => {
            error!("Erro: {}", e);
            trace!("Programa finalizado com erro");
            trace!(
                "Codigo de erro retornado ao sistema operacional: {}",
                e.exit_code()
            );
            std::process::exit(e.exit_code()); // Retorna o codigo de erro especifico
        }
    }
}
