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


| Documento | Descrição |
|-----------|-----------|
| [Guia Rapido de Uso](https://github.com/fvilante/video_frame_saver/blob/main/docs/GUIA_DE_USO.txt) | Instruções detalhadas sobre como utilizar o programa. |
| [Manual Completo](https://github.com/fvilante/video_frame_saver/blob/main/docs/MANUAL_COMPLETO.md) | Documentação técnica e instruções avançadas. |
| [Códigos de Erro](https://github.com/fvilante/video_frame_saver/blob/main/docs/CODIGOS_DE_ERRO.txt) | Lista de códigos de erro retornados pelo programa e possíveis soluções. |
| [Como Compilar](https://github.com/fvilante/video_frame_saver/blob/main/docs/COMO_COMPILAR.md) | Instruções para compilar o programa a partir do código-fonte. |
| [Histórico de Mudanças](https://github.com/fvilante/video_frame_saver/blob/main/CHANGELOG.md) | Registro de mudanças e melhorias do projeto. |