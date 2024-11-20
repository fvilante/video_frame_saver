# Compilação, Execução e Distribuição.


## 1) Instalar Compilador de linguagem Python na maquina
	
Faca download pelo site: https://www.python.org/downloads/windows/

Atende para instalar o python para o sistema operacional correspondente ao que você deseja.

Este programa foi testado na versão 3.11.9 do python. Porem deve funcionar em qualquer versão superior tbm.

Para assegurar que esta tudo ok com a instalação digite `python --version` na linha de comando.

NOTA: Pode ser necessário ajustar a variável de ambiente PATH para conter o diretório onde o python esta instalado.

```
> python --version
Python 3.11.9
```

## 2) Instalar dependências opencv usando python

O python pode instalar automaticamente o opencv para ser acessado através da linguagem, basta digitar o comando:

```
> python -m pip install opencv-contrib-python
```


## 3) Rodar o programa de modo interpretado

Neste momento o script já pode ser executado de modo interpretado, bastando digitar:

```
> python python_opencv.py
```

## 4) Gerar o executável usando PyInstaler.

Para gerar o executável instale o programa `PyInstaler` digitando:

```
> python -m pip install pyinstaller
```

Em seguida crie o executável digitando:

```
> pyinstaller --onefile python_opencv.py
```

Ao final da execucacao do comando acima o executável estará localizado na sub-pasta `dist` que estará no mesmo diretório do seu script Python. Por exemplo:

```
dist/
    python_opencv.exe
```

## 5) Distribuir o executável

O arquivo `python_opencv.exe` pode ser distribuído através de pen-drive ou qualquer outro meio. Ele é executado sem necessidade de .DLLs ou outras dependências externas. 

Vale apenas lembrar que se for compilado no Windows então será um executável que apenas rodará neste mesmo sistema operacional e não em outros.
