import cv2

def main():
    camera_index = 1  # 0 = webcam padrÃ£o; 1 = cÃ¢mera USB
    cam = cv2.VideoCapture(camera_index)

    # Verifica se a cÃ¢mera foi aberta com sucesso
    if not cam.isOpened():
        print("Erro ao acessar a cÃ¢mera!")
        return

    # ConfiguraÃ§Ãµes de resoluÃ§Ã£o (opcional)
    cam.set(cv2.CAP_PROP_FRAME_WIDTH, 4000)
    cam.set(cv2.CAP_PROP_FRAME_HEIGHT, 3000)

    print(f"ResoluÃ§Ã£o atual: {cam.get(cv2.CAP_PROP_FRAME_WIDTH)}x{cam.get(cv2.CAP_PROP_FRAME_HEIGHT)}")

    cv2.namedWindow("Webcam")

    if not cam.isOpened():
        print("Erro: NÃ£o foi possÃ­vel acessar a cÃ¢mera. Verifique se ela estÃ¡ conectada e nÃ£o estÃ¡ sendo usada por outro programa.")
        exit()

    while True:
        ret, frame = cam.read()  # Captura um frame da webcam
        if not ret:
            print("Erro ao capturar frame!")
            break

        # Redimensiona o frame para exibiÃ§Ã£o (opcional)
        resized_frame = cv2.resize(frame, (800, 600))

        # Exibe o frame
        cv2.imshow("Webcam", resized_frame)

        # Aguarda uma tecla pressionada
        key = cv2.waitKey(10)
        if key == 27:  # Tecla ESC para sair
            print("Pressionou ESC. Encerrando...")
            break
        elif key == 32:  # Tecla EspaÃ§o para salvar o frame
            print("Salvando imagem...")
            cv2.imwrite("imagem_capturada.bmp", frame)
            print("Imagem salva como 'imagem_capturada.bmp'.")
            break

    # Libera os recursos
    cam.release()
    cv2.destroyAllWindows()

if __name__ == "__main__":
    main()
