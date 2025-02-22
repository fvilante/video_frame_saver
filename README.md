# video\_frame\_saver

## Introdução

**video\_frame\_saver** é uma ferramenta de linha de comando para captura de imagens a partir de câmeras conectadas ao computador. O programa exibe o fluxo de vídeo da câmera em tempo real e permite ao usuário capturar um frame pressionando a tecla **espaço** e salvando o frame em um arquivo localmente.

A ferramenta suporta múltiplos parâmetros de configuração, incluindo a seleção da câmera, ajustes de resolução e escala da imagem, além de funcionalidades para listar câmeras disponíveis e detectar resoluções suportadas. Caso a resolução especificada não seja suportada, a câmera ajustará automaticamente para a mais próxima disponível.

O programa pode ser utilizado tanto para testes rápidos quanto para automação de captura de imagens em ambientes controlados. Pressionar **ESC** encerra a execução sem salvar nenhuma imagem.

Compatível com sistemas operacionais que suportam **Rust**, incluindo **Windows**, **Linux** e **ARM**.

## Funcionalidades Principais

- Captura o vídeo de uma câmera conectada.
- Exibe o vídeo em tempo real em uma janela.
- Permite salvar um frame pressionando a tecla **espaço**.
- Suporte a escalonamento da imagem capturada.
- Lista as câmeras acessíveis.
- Detecta e exibe as resoluções suportadas pela câmera.
- Permite definir manualmente a resolução da câmera ao capturar a imagem.
- Autodetecta as resoluções suportadas pela câmera.
- Ajusta automaticamente para a resolução mais próxima disponível caso a especificada não seja suportada.

## Uso Básico

Para executar o programa com as configurações padrão:

```sh
video_frame_saver.exe
```

Pressione **espaço** para salvar um frame capturado.
Pressione **ESC** para encerrar sem salvar.

Para visualizar as opções disponíveis:

```sh
video_frame_saver.exe --help
```

## Opções de Linha de Comando

```sh
Usage: video_frame_saver.exe [OPTIONS]
```

### Principais Opções

| Opção                     | Descrição                                                   |
| ------------------------- | ----------------------------------------------------------- |
| `-c, --camera-index <N>`  | Define o índice da câmera a ser usada (default: 0).         |
| `-f, --image-name <NOME>` | Nome do arquivo de saída (default: `imagem_capturada.bmp`). |
| `-x, --scale-x <FATOR>`   | Define o fator de escala horizontal (default: 1).           |
| `-y, --scale-y <FATOR>`   | Define o fator de escala vertical (default: 1).             |
| `-l, --list`              | Lista todas as câmeras acessíveis.                          |
| `--detect-resolutions`    | Detecta e lista as resoluções suportadas pela câmera.       |
| `--width <WIDTH>`         | Define a largura desejada da câmera.                        |
| `--height <HEIGHT>`       | Define a altura desejada da câmera.                         |
| `--verbose`               | Ativa o modo de depuração (exibe informações detalhadas).   |
| `-h, --help`              | Exibe o menu de ajuda.                                      |
| `-V, --version`           | Exibe a versão do programa.                                 |

## Exemplos de Uso

### Listar todas as câmeras conectadas

```sh
video_frame_saver.exe --list
```

### Detectar resoluções suportadas pela câmera 0

```sh
video_frame_saver.exe --detect-resolutions -c 0
```

### Capturar imagem com escala ajustada

```sh
video_frame_saver.exe -x 2 -y 2
```

### Capturar imagem com resolução personalizada

```sh
video_frame_saver.exe --width 1920 --height 1080
```

### Especificar um nome personalizado para a imagem

```sh
video_frame_saver.exe -f minha_imagem.bmp
```

## Formatos de Imagem Suportados

O formato do arquivo salvo é inferido pela extensão do nome fornecido no parâmetro `-f`. Os principais formatos suportados são:

| Formato | Extensão        | Compressão | Tipo de Compressão |
| ------- | --------------- | ---------- | ------------------ |
| BMP     | `.bmp`          | Não        | Sem perda          |
| JPEG    | `.jpg`, `.jpeg` | Sim        | Com perda          |
| PNG     | `.png`          | Sim        | Sem perda          |


## Ajuste Automático de Resolução

Se uma resolução não suportada for fornecida, a câmera ajustará automaticamente para a mais próxima disponível. Para descobrir quais resoluções são suportadas:

```sh
video_frame_saver.exe --detect-resolutions
```

>**IMPORTANTE**: O parametro `--detect-resolutions` busca pelas resolucoes mais populares no mundo, se a camera possuir uma resolução exótica, esta resolução pode nao ser detectada. Contudo ainda sim voce pode utilizar os parametros `--height` e `--width` para tentar _forçar_ na camera esta resolução exotica. Caso ela nao aceite, ela irá simplesmente selecionar a resolucao mais proxima disponível.


## Versão Atual

Para conferir a versão instalada:

```sh
video_frame_saver.exe --version
```

### Dependências e Caminho da DLL

Se o programa não iniciar corretamente, verifique se a **.dll** necessária está no mesmo diretório do executável ou se pode ser acessada via **PATH**.

## Informações Adicionais

- O programa **não altera permanentemente as configurações da câmera**.
- Para informações mais detalhadas, consulte o repositório no GitHub.


## Relatório de Bugs
Os bugs conhecidos podem ser visualizados [aqui](https://github.com/fvilante/video_frame_saver/issues) 

## Outras informações
As documentações abaixos sao distribuidas junto com os arquivos binarios:
- [Documentacao dos codigos de erros](https://raw.githubusercontent.com/fvilante/video_frame_saver/refs/heads/main/resources/CODIGOS_DE_ERRO.txt)
- [Guia de uso](https://raw.githubusercontent.com/fvilante/video_frame_saver/refs/heads/main/resources/GUIA_DE_USO.txt)

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

Vilante Labs 2024
