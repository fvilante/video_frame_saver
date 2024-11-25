use opencv::{core, highgui, imgcodecs, imgproc, prelude::*, videoio, Result};

// site da camera = http://www.beleng.com.br/capa.asp?pi=produto&proid=853

fn main() -> Result<()> {
    let camera_index: i32 = 0; // 0 = webcam padrao do laptop ; 1 = camera adicional connectada a USB
    let mut cam = videoio::VideoCapture::new(camera_index, videoio::CAP_ANY)?;
    let w = cam.get(videoio::CAP_PROP_FRAME_WIDTH)?;
    let h = cam.get(videoio::CAP_PROP_FRAME_HEIGHT)?;
    println!("Stream da camera: W_{}xH_{}", w, h);
    //
    cam.set(videoio::CAP_PROP_FRAME_WIDTH, 4000.0)?;
    cam.set(videoio::CAP_PROP_FRAME_HEIGHT, 3000.0)?;
    //
    let w = cam.get(videoio::CAP_PROP_FRAME_WIDTH)?;
    let h = cam.get(videoio::CAP_PROP_FRAME_HEIGHT)?;
    println!("Stream da camera reconfigurado: W_{}xH_{}", w, h);
    highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;
    let mut frame = Mat::default();
    let mut frame_resized = Mat::default();
    loop {
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            //println!("Tamanho do frame: W_{}xH_{}",frame.size()?.width, frame.size()?.height);
            //
            let value = 500;
            imgproc::resize(
                &frame,
                &mut frame_resized,
                core::Size::new(0, 0),
                1.0,
                1.0,
                50,
            );
            highgui::imshow("window", &mut frame_resized)?;
        }
        let key = highgui::wait_key(10)?;

        let params = core::Vector::new();
        // params.push(1000);
        if key > 0 && key != 255 {
            if key == 27 {
                println!("Pressionado tecla ESC, nenhuma imagem será salva. Para salvar pressione uma tecla diferente de ESC");
                break;
            }
            println!("Iniciando rotina de gravacao...");
            let res = imgcodecs::imwrite("imagem_capturada.bmp", &frame, &params); //TODO: testar também TIFF/JPG/PNG/DICOM
            match res {
                Ok(_) => println!("Gravacao bem sucedida!"),
                Err(e) => println!("Erro de gravacao: {e}"),
            }
            break;
        }
    }

    Ok(())
}
