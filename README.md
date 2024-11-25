# video_frame_recorder

## Introdução

Este programa serve para ler sinal de video de camera externa e salvar frames selecionados pelo usuario em um arquivo de texto.

Voce pode baixar uma versao ja pre-compilada do executavel neste link: https://github.com/fvilante/video_frame_saver/releases

Este documento ensina como compilar, executar e criar o binario para distribuicao.

Todos os comandos elencados neste texto devem ser digitados na linha de comando do sistema operacional. (exemplo: Powershell no Windows).

## Funcionamento esperado do programa

Esta versao deve ser capaz de rodar em qualquer sistema operacional que suporte Python (ie: Windows, Linux, etc), e ler o sinal de video de uma camera externa conectada ao computador, normalmente a porta USB, e imediatamente mostrar a imagem capturada em uma janela na tela.

Ao pressionar a tecla de espaço o programa irá salvar no mesmo diretorio do executavel um arquivo de nome `imagem_capturada.bmp` que contem o frame no momento em que a tecla espaco foi pressionada, e automaticamente o programa em seguida é encerrado.

### Bugs conhecidos

1) Se vc tentar fechar ò aplicativo clicando no "X" no canto superior da janela, o programa nao encerra, e sim apenas quando aperta a tecla espaco.

## Como compilar localmente

## Baixar o codigo-fonte

Instale o [git para windows](https://git-scm.com/downloads/win). E confira se a instalacao foi bem sucedida digitando `git --version` na linha de comando.

Mude para o diretorio que deseja salvar e execute o comando `git clone` assim:

```powershell
> cd meu_diretorio
> git clone https://github.com/fvilante/video_frame_saver.git
> cd video_frame_saver
> dir 
``` 

## Instalação das dependencias

Além da linguagem Rust, precisaremos da biblioteca OpenCV instalada na máquina, para instalar estas dependencias uma forma pratica é utilizando o comando `chocolatey`. Ele funciona como uma especie de "Google Play" para o Powershell e permitirá instalar as demais dependencias de modo pratico.

Os comandos abaixo devem ser digitados no Poweshell, que é a linha de comando do Windows. Lembre-se que em alguns casos pode ser necessario abrir uma linha de comando em modo _administrador_.

### 1) Instale [chocolatey](https://chocolatey.org/install).

Para verificar que a instalação está correta digite:

```powershell
> choco --version
2.3.0
```

### 2) Instalee `OpenCV` and `LLVM`, digitando o comando: 


```powershell
> choco install llvm opencv
```

Voce pode verificar a instalacao do llvm digitando:

```powershell
> clang --version
clang version 19.1.3
Target: x86_64-pc-windows-msvc
Thread model: posix
InstalledDir: C:\Program Files\LLVM\bin
```

Para verificar o OpenCV vale notar que o diretorio padrao onde a livraria OpenCV é instalada é em `c:\tools`.

### 3) Garanta que estas variaveis de ambiente abaixo estao definidas. 

Esta etapa é necessária para que o compilador Rust encontre a livraria OpenCV e seja capaz de linkar o executavel com a DLL do OpenCV.

Digite:

```powershell
> $env:OPENCV_INCLUDE_PATHS = "C:\tools\opencv\build\include"
> $env:OPENCV_LINK_LIBS = "opencv_world4100" # nome do arquivo cuja extensao é .dll (porem a extensoa nao é colocada)
> $env:OPENCV_LINK_PATHS = "C:\tools\opencv\build\x64\vc16\lib"
> $env:Path += ";C:\tools\opencv\build\x64\vc16\bin" # o endereco do arquivo $env:OPENCV_LINK_LIBS
```

NOTA: Da forma acima, as variaveis de ambiente apenas serão validas durante a sessao atual do Powershell, e serão perdidas quando a sessao atual for fechada. Existe uma forma de fazer estas variaveis nao volateis, que nao será tratado aqui.

### 4) Instale a linguagem de programacao [Rust](https://www.rust-lang.org/tools/install).

### 5) Compile

Uma vez tendo todas as dependencias instaladas, e estando no diretorio raiz do codigo-fonte (o mesmo diretorio que contem o aquivo `cargo.toml`), digite:

```powershell
> cargo clean # para limpar os arquivos relativos a ultima compilacao (opcional)
> cargo build # para compilar, ou alternativamente `cargo run` que compilará e automaticamente executará o programa.
```

### 5) Distribua

O executavel compilado fica no subdiretorio `target/debug`. Lembre-se de que se for distribuir para outro computador, é necessario assegurar que o arquivo `.dll` cujo nome esta na variavel de ambiente `$env:OPENCV_LINK_LIBS` (ver acima) esteja ou no mesmo diretorio do executavel, ou então que esteja em algum diretorio listado na variavel de ambiente `PATH`. Caso contrario o programa retornará erro, indicando que a DLL nao pode ser encontrada.

-------------------------------------

Vilante Labs 2024
