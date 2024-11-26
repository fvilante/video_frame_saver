use opencv::{core, highgui, imgcodecs, imgproc, prelude::*, videoio, Result};

/// Configurações iniciais do programa
const DEFAULT_CAMERA_INDEX: i32 = 0;

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
    println!(
        "Câmeras acessíveis: {:?}\nCâmeras não acessíveis: {:?}",
        cameras_disponiveis,
        range
            .filter(|index| !cameras_disponiveis.contains(index))
            .collect::<Vec<_>>()
    );
}

struct CameraSize {
    width: f64,
    height: f64,
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

fn camera_set_size(camera: &mut videoio::VideoCapture, camera_size: CameraSize) -> Result<()> {
    camera.set(videoio::CAP_PROP_FRAME_WIDTH, camera_size.width)?;
    camera.set(videoio::CAP_PROP_FRAME_HEIGHT, camera_size.height)?;
    Ok(())
}

fn main() -> Result<()> {
    //imprime_lista_de_cameras_acessiveis(0..5);

    let mut camera = initialize_camera(DEFAULT_CAMERA_INDEX)
        .unwrap_or_else(|e| panic!("Erro ao inicializar a câmera: {}", e));

    let camera_size = camera_get_size(&camera)
        .unwrap_or_else(|e| panic!("Erro ao obter o tamanho da câmera: {}", e));

    highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;
    let mut frame = Mat::default();
    let mut frame_resized = Mat::default();
    loop {
        // Le frame da camera
        if let Err(e) = camera.read(&mut frame) {
            eprintln!("Erro ao ler frame: {}", e);
            continue;
        }

        // redimensiona imagem da camera e mostra na tela ja redimensionado
        if frame.size()?.width > 0 {
            imgproc::resize(
                &frame,
                &mut frame_resized,
                core::Size::new(0, 0),
                1.0,
                1.0,
                imgproc::INTER_LINEAR,
            );
            highgui::imshow("window", &mut frame_resized)?;
        }
        let key = highgui::wait_key(10)?;
        let params = core::Vector::new();
        if key > 0 && key != 255 {
            if key == 27 {
                println!("Pressionado tecla ESC, nenhuma imagem será salva.");
                break;
            }
            println!("Iniciando rotina de gravação...");
            match imgcodecs::imwrite("imagem_capturada.bmp", &frame, &params) {
                Ok(_) => println!("Gravação bem-sucedida!"),
                Err(e) => eprintln!("Erro ao salvar imagem: {}", e),
            }
            break;
        }
    }

    Ok(())
}
