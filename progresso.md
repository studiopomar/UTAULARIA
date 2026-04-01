# Status do Projeto UTAULARIA

Este documento descreve o estado atual do desenvolvimento do software UTAULARIA.

## O que já foi implementado

### Estrutura do Projeto

- Inicialização do ambiente Rust.
- Configuração de dependências (eframe para GUI, serde para dados, rfd para diálogos de arquivos).

### Modelagem de Dados

- Criação da estrutura de dados para Voicebanks (Nome, Criador, Tipo, Idioma, Link, Descrição).

### Interface Gráfica (GUI)

- Janela principal com sistema de painéis.
- Painel Lateral: Lista de voicebanks cadastrados com botões de Adicionar e Salvar.
- Painel Central: Editor detalhado para o voicebank selecionado.
- Botão "Gerar README" para automatizar a atualização do repositório.
- Botão "Validar Pasta" para checagem técnica de arquivos.

### Persistência e Lógica de Arquivos

- Sistema de salvamento e carregamento via arquivos YAML (voicebanks.yaml).
- Algoritmo de atualização automática da tabela no arquivo readme.md.
- Validador técnico que verifica a presença de oto.ini, arquivos .wav e documentação (readme/characther).

---

## O que ainda será feito

### Correções Técnicas

- Resolução de erros de compilação pendentes relacionados ao novo sistema de validação.

### Melhorias de Interface

- Refinamento visual para uma aparência mais moderna e profissional ("Premium").
- Adição de ícones e feedback visual mais claro para o usuário.
- Tratamento de estados vazios (quando não há voicebanks carregados).

### Funcionalidades Adicionais

- Opção para editar a ordem dos voicebanks na lista.
- Exportação de metadados em outros formatos se necessário.
- Testes de robustez no gerador do README para evitar sobrescritas acidentais em formatos inesperados.
- Implementação de um sistema de busca/filtro na lista lateral.
