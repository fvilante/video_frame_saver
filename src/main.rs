use clap::Parser;
use opencv::{core, highgui, imgcodecs, imgproc, prelude::*, videoio, Result};
use prettytable::{color, Attr, Cell, Row, Table};

/// Configurações do programa obtidas via linha de comando
#[derive(Parser, Debug)]
#[command(
    author = "Flavio Vilante <fvilante1@gmail.com>",
    version = "1.4.6",
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

/// Inicializa a câmera com o índice especificado
fn initialize_camera(camera_index: i32) -> Result<videoio::VideoCapture> {
    let mut camera = videoio::VideoCapture::new(camera_index, videoio::CAP_ANY)?;
    if !camera.is_opened()? {
        return Err(opencv::Error::new(
            core::StsError,
            format!(
                "Não foi possível abrir a câmera com o índice: {}",
                camera_index
            ),
        ));
    }
    Ok(camera)
}

/// Libera os recursos associados à câmera
fn release_camera(mut camera: videoio::VideoCapture) -> Result<()> {
    camera.release()
}

fn lista_cameras_acessiveis(range: impl Iterator<Item = i32>) -> Vec<i32> {
    range
        .filter_map(|index| {
            initialize_camera(index)
                .map(|camera| {
                    let _ = release_camera(camera);
                    index
                })
                .ok()
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

fn camera_set_size(camera: &mut videoio::VideoCapture, config: &Config) {
    if let (Some(w), Some(h)) = (config.width, config.height) {
        if config.verbose {
            println!("Definindo resolução da câmera para {}x{}", w, h);
        }
        camera.set(videoio::CAP_PROP_FRAME_WIDTH, w as f64);
        camera.set(videoio::CAP_PROP_FRAME_HEIGHT, h as f64);
    } else if config.verbose {
        println!("Usando resolução padrão da câmera.");
    }
}

fn camera_get_size(camera: &videoio::VideoCapture) -> Result<CameraSize> {
    let width = camera
        .get(videoio::CAP_PROP_FRAME_WIDTH)
        .map_err(|e| opencv::Error::new(core::StsError, format!("Erro ao obter largura: {}", e)))?;
    let height = camera
        .get(videoio::CAP_PROP_FRAME_HEIGHT)
        .map_err(|e| opencv::Error::new(core::StsError, format!("Erro ao obter altura: {}", e)))?;
    if width <= 0.0 || height <= 0.0 {
        return Err(opencv::Error::new(
            core::StsError,
            "Dimensões inválidas: largura e altura devem ser maiores que zero".to_string(),
        ));
    }
    Ok(CameraSize { width, height })
}

/// Exibe os frames capturados da câmera, redimensionados com os fatores especificados
fn display_camera_feed(camera: &mut videoio::VideoCapture, config: &Config) -> Result<()> {
    highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;
    let mut frame = Mat::default();
    let mut frame_resized = Mat::default();

    loop {
        if let Err(e) = camera.read(&mut frame) {
            eprintln!("Erro ao ler frame: {}", e);
            continue;
        }

        if frame.size()?.width > 0 {
            imgproc::resize(
                &frame,
                &mut frame_resized,
                core::Size::new(0, 0),
                config.scale_x,
                config.scale_y,
                imgproc::INTER_LINEAR,
            );
            highgui::imshow("window", &mut frame_resized)?;
        }

        let key = highgui::wait_key(10)?;

        if process_key_input(key, &frame, &config.image_name)? {
            break;
        }
    }

    Ok(())
}

/// Processa a entrada do teclado durante a exibição do feed da câmera
fn process_key_input(key: i32, frame: &Mat, image_name: &str) -> Result<bool> {
    if key > 0 && key != 255 {
        if key == 27 {
            println!("Pressionado tecla ESC, nenhuma imagem será salva.");
            return Ok(true);
        }

        println!("Iniciando rotina de gravação...");
        let params = core::Vector::new();
        match imgcodecs::imwrite(image_name, frame, &params) {
            Ok(_) => println!("Gravação bem-sucedida! Arquivo salvo como: {}", image_name),
            Err(e) => eprintln!("Erro ao salvar imagem: {}", e),
        }
        return Ok(true);
    }
    Ok(false)
}

fn listar_resolucoes_suportadas(camera: &mut videoio::VideoCapture) -> Vec<(i32, i32)> {
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
            .unwrap();
        camera
            .set(videoio::CAP_PROP_FRAME_HEIGHT, *h as f64)
            .unwrap();

        let width = camera.get(videoio::CAP_PROP_FRAME_WIDTH).unwrap() as i32;
        let height = camera.get(videoio::CAP_PROP_FRAME_HEIGHT).unwrap() as i32;

        if width == *w && height == *h {
            resolucoes_suportadas.push((*w, *h));
        }
    }

    resolucoes_suportadas
}

fn main() -> Result<()> {
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
        let resolucoes = listar_resolucoes_suportadas(&mut camera);
        println!("Resoluções suportadas pela câmera:");
        for (w, h) in resolucoes {
            println!("{}x{}", w, h);
        }
        return Ok(());
    } else {
        // Seta dimensoes da camera
        camera_set_size(&mut camera, &config);

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
