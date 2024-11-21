# video_frame_recorder

## Introdução

Este programa serve para ler sinal de video de camera externa e salvar frames selecionados pelo usuario em um arquivo de texto.

Voce pode baixar uma versao ja pre-compilada do executavel neste link: https://github.com/fvilante/video_frame_saver/releases

Este documento ensina como compilar, executar e criar o binario para distribuicao.

Todos os comandos elencados neste texto devem ser digitados na linha de comando do sistema operacional. (exemplo: Powershell no Windows).

## Funcionamento esperado do programa

Esta versao deve ser capaz de rodar em qualquer sistema operacional que suporte Python (ie: Windows, Linux, etc), e ler o sinal de video de uma camera externa conectada ao computador, normalmente a porta USB, e imediatamente mostrar a imagem capturada em uma janela na tela.

Ao pressionar a tecla de espaço o programa irá salvar no mesmo diretorio do executavel um arquivo de nome `imagem_capturada.bmp` que contem o frame no momento em que a tecla espaco foi pressionada, e automaticamente o programa em seguida é encerrado.

```
Bugs conhecidos: 
1) Se vc tentar fechar ò aplicativo clicando no "X" no canto superior da janela, o programa nao encerra, e sim apenas quando aperta a tecla espaco.
```

*IMPORTANTE*: Este programa nao depende de nenhuma biblioteca externa, e pode ser distribuido em um pendrive com apenas o arquivo executavel. Nao é necessario instalador.

# Compilação, Execução e Distribuição.

## Baixar o codigo-fonte

Instale o [git para windows](https://git-scm.com/downloads/win). E confira se a instalacao foi bem sucedida digitando `git --version` na linha de comando.

Mude para o diretorio que deseja salvar e execute o comando `git clone` assim:

```powershell
> cd meu_diretorio
> git clone https://github.com/fvilante/video_frame_saver.git
> cd video_frame_saver
> dir 
``` 


## 1) Instalar Compilador de linguagem Python na maquina
	
Faca download pelo site: https://www.python.org/downloads/windows/

Atende para instalar o python para o sistema operacional correspondente ao que você deseja.

Este programa foi testado na versão 3.11.9 do python. Porem deve funcionar em qualquer versão superior tbm.

Para assegurar que esta tudo ok com a instalação digite `python --version` na linha de comando.

NOTA: Pode ser necessário ajustar a variável de ambiente PATH para conter o diretório onde o python esta instalado.

```powershell
> python --version
Python 3.11.9
```

## 2) Instalar dependências opencv usando python

O python pode instalar automaticamente o opencv para ser acessado através da linguagem, basta digitar o comando:

```powershell
> python -m pip install opencv-contrib-python
```


## 3) Rodar o programa de modo interpretado

Neste momento o script já pode ser executado de modo interpretado, bastando digitar:

```powershell
> python python_opencv.py
```

## 4) Gerar o executável usando PyInstaler.

Para gerar o executável instale o programa `PyInstaler` digitando:

```powershell
> python -m pip install pyinstaller
```

Em seguida crie o executável digitando:

```powershell
> pyinstaller --onefile python_opencv.py
```

Ao final da execucacao do comando acima o executável estará localizado na sub-pasta `dist` que estará no mesmo diretório do seu script Python. Por exemplo:

```powershell
dist/
    python_opencv.exe
```

## 5) Distribuir o executável

O arquivo `python_opencv.exe` pode ser distribuído através de pen-drive ou qualquer outro meio. Ele é executado sem necessidade de .DLLs ou outras dependências externas. 

Vale apenas lembrar que se for compilado no Windows então será um executável que apenas rodará neste mesmo sistema operacional e não em outros.


---
Vilante Labs 2024
