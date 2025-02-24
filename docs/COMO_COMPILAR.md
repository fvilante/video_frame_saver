# video\_frame\_saver

## Como compilar localmente

Todos os comandos elencados abaixo devem ser digitados na linha de comando do sistema operacional. (exemplo: Powershell no Windows).

A partir daqui este documento irá ensina como compilar, executar e criar o binario para distribuicao do programa.

Porém voce pode baixar uma versao ja pre-compilada do executavel neste link: https://github.com/fvilante/video_frame_saver/releases.

Se contudo quiser compilar o programa localmente na sua maquina siga os passos a seguir:

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

### 2) Instale `OpenCV` and `LLVM`, digitando o comando: 

Para instalar o [LLVM](https://llvm.org/):

```powershell
> choco install llvm --version=19.1.3
```

Para instalar o [OpenCV](https://opencv.org/):

```powershell
> choco install opencv --version=4.10.0 
```

Voce pode verificar a instalacao do llvm digitando:

```powershell
> clang --version
clang version 19.1.3
Target: x86_64-pc-windows-msvc
Thread model: posix
InstalledDir: C:\Program Files\LLVM\bin
```

E verificar a instalacao do OpenCV digitando:

```powershell
choco list opencv
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

NOTA: Da forma acima, as variaveis de ambiente apenas serão validas APENAS durante a sessao atual do Powershell, e serão perdidas quando a sessao atual for fechada. Existe uma forma de fazer estas variaveis nao volateis, que nao será tratado aqui.

### 4) Instale a linguagem de programacao [Rust](https://www.rust-lang.org/tools/install).

### 5) Compile

Uma vez tendo todas as dependencias instaladas, e estando no diretorio raiz do codigo-fonte (o mesmo diretorio que contem o aquivo `cargo.toml`), digite:

```powershell
> cargo clean # para limpar os arquivos relativos a ultima compilacao (opcional)
> cargo build # para compilar, ou alternativamente `cargo run` que compilará e automaticamente executará o programa.
```

### 5) Distribua

O executavel compilado fica no subdiretorio `target/debug`. Lembre-se de que se for distribuir para outro computador, é necessario assegurar que o arquivo `.dll` cujo nome esta na variavel de ambiente `$env:OPENCV_LINK_LIBS` (ver acima) esteja ou no mesmo diretorio do executavel, ou então que esteja em algum diretorio listado na variavel de ambiente `PATH`. Caso contrario o programa retornará erro, indicando que a DLL nao pode ser encontrada.


## Compilação na nuvem

Caso nao queira compilar localmente, você pode compilar na nuvem.

Este repositório conta com um processo automatizado de geração de **releases** no GitHub, utilizando **GitHub Actions**.  

Sempre que um novo **commit** é enviado (_push_) para o repositório e contém uma **tag de versão** iniciada por `v` (por exemplo, `v1.4.7`), um processo de compilacao automatica é iniciado na nuvem e uma vez encerrada uma nova [release](https://github.com/fvilante/video_frame_saver/releases).  é disponibilizada.

Esta é uma forma de compilação e distribuição feita automaticamente na nuvem.

O arquivo de configuração responsável por essa automação pode ser encontrado aqui:  
[release.yml](https://github.com/fvilante/video_frame_saver/blob/main/.github/workflows/release.yml).

-------------------------------------

Vilante Labs (C) 2025