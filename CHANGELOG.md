## Registro de Mudancas
## [unreleased]

## [1.4.8] - 2025-02-22

### Feat
- Programa agora retorna codigo de erro ao sistema operacional para facilitar integracao.
- Adicionado arquivos junto a distribuicao: 
    - "GUIA_DE_USO.txt" para orientar o usuario.
    - "CODIGOS_DE_ERRO.txt" para documentar os tipos e codigos de erros gerados pelo programa.

### Ci
- Update Rust toolchain to 1.85.0
- Permite acionar o build manualmente pelo site do GitHub.

### Docs
- Revisao da documentacao do projeto.
- Documentacao dos codigos de erro no README.


---

## [1.4.7] - 2025-02-14

### Build
- Esta versao para todos os efeitos praticos é igual a versao 1.4.6.
- Aperfeicoado a forma como o codigo-fonte é compilada e distribuido.


---

## [1.4.6] - 2025-02-14

### Feat
- Adicionado funcionalidade para redefinir e/ou detectar resolucoes admitidas na camera.

---

## [1.4.4] - 2024-11-26

### **Resumo de Mudancas das Versoes Anteriores**

Esta versao e as anteriores introduziram as principais funcionalidades do **video_frame_saver**, incluindo:

- **Estrutura Inicial do Projeto**:  
  - Primeira versao escrita em **Rust**, substituindo uma versao anterior em Python.  
  - Organizacao inicial do projeto com `cargo` e configuracoes do `.gitignore`.

- **Suporte a OpenCV**:  
  - Configuracao de captura de video utilizando OpenCV.  
  - Implementacao de leitura e escrita de frames.

- **Linha de Comando**:  
  - Implementacao de CLI para configurar a camera e capturar frames.  
  - Parametros para escolher **indice da camera, resolucao e escala da imagem**.

- **Automacao e Distribuicao**:  
  - Integracao com **GitHub Actions** para automatizar build e releases.  
  - Geracao de binarios para Windows com empacotamento via ZIP.

- **Melhorias Progressivas**:  
  - **Deteccao de cameras disponiveis** conectadas ao sistema.  
  - **Suporte para leitura de parametros da linha de comando**.  
  - **Geracao automatica de releases** com executavel e dependencias.  

A partir da versao **1.4.4**, o foco passou a ser **melhorar a estabilidade e adicionar funcionalidades especificas**.

---

Vilante Labs (c) 2025
