# todo_cli

Gerenciador de tarefas em linha de comando escrito em **Rust**.

Projeto da disciplina de Tópicos em Programação 3 (UTFPR). O objetivo é demonstrar
os recursos centrais da linguagem em uma aplicação pequena e executável:

| Recurso do Rust | Onde aparece |
| --- | --- |
| `struct` e `enum` com dados | `Tarefa`, `Prioridade`, `Comando` |
| `match` exaustivo | despacho de comandos em `src/main.rs` |
| `Result` e operador `?` | toda a propagação de erros, sem exceções |
| Ownership e borrowing | `&mut self` nas operações, `&Tarefa` na listagem |
| Traits (`Display`, `Ord`) e `derive` | formatação e ordenação por prioridade |
| Iteradores e closures | `filter`, `map`, `position`, `sort_by_key` |
| Módulos | `src/tarefa.rs` e `src/menu.rs` separados de `src/main.rs` |
| Entrada e saída (`std::io`) | leitura do teclado no modo interativo |
| Crates externas | `serde` e `serde_json` para gravar em JSON |
| Testes integrados ao compilador | `cargo test`, 8 testes |

## Instalação

1. Instale o Rust pelo [rustup](https://rustup.rs). No Windows, aceite a instalação
   das ferramentas de build C++ quando o instalador pedir.
2. Confirme a instalação:

```bash
cargo --version
```

3. Clone o repositório e compile:

```bash
cargo build --release
```

O executável fica em `target/release/todo_cli` (`todo_cli.exe` no Windows).

## Configuração

Não há configuração: as tarefas são gravadas no arquivo `tarefas.json`, criado
automaticamente no diretório onde o programa é executado. Para começar do zero,
apague esse arquivo.

## Uso

O programa tem duas interfaces, e as duas gravam no mesmo arquivo.

### Modo interativo

Rodar sem argumentos abre um menu que fica ativo até você escolher sair:

```bash
cargo run
```

```
=== TODO CLI ===
1  Adicionar tarefa
2  Listar tarefas
3  Concluir tarefa
4  Remover tarefa
0  Sair
opcao:
```

O programa pergunta os dados de cada operação (descrição, prioridade, id). Opção
inexistente, id não numérico e id inexistente são avisados na tela e o menu
continua. Para sair, escolha `0` ou pressione Ctrl+Z (Windows) / Ctrl+D (Linux).

### Modo por argumentos

Durante o desenvolvimento, use `cargo run --` antes dos argumentos:

```bash
cargo run -- add "Terminar o manual tecnico" alta
```

Com o binário já compilado:

```bash
todo_cli add "Estudar ownership"
```

| Comando | Descrição |
| --- | --- |
| `add <descrição> [alta\|media\|baixa]` | adiciona uma tarefa (padrão: `media`) |
| `list [--pendentes]` | lista as tarefas ordenadas por prioridade |
| `done <id>` | marca a tarefa como concluída |
| `rm <id>` | remove a tarefa |
| `help` | mostra a ajuda |

Exemplo de sessão completa:

```bash
todo_cli add "Terminar o manual tecnico" alta
todo_cli add "Estudar ownership"
todo_cli add "Comprar cafe" baixa
todo_cli done 2
todo_cli list
```

Saída:

```
[ ]   1  Terminar o manual tecnico                (alta)
[ ]   3  Comprar cafe                             (baixa)
[x]   2  Estudar ownership                        (media)
```

Erros são informados na saída de erro e o programa termina com código 1:

```
$ todo_cli done 99
erro: tarefa 99 nao encontrada
```

## Testes

```bash
cargo test
```

São 10 testes cobrindo a interpretação dos argumentos (comando desconhecido, id não
numérico, prioridade inválida), as operações da lista (ids sequenciais, conclusão,
remoção de id inexistente, ordenação por prioridade) e a leitura das opções do menu.

## Estrutura

```
src/
  main.rs     argumentos de linha de comando, despacho e saída
  menu.rs     modo interativo: menu, leitura do teclado e validação
  tarefa.rs   regras de negócio: Tarefa, Prioridade, Lista e persistência
```

As duas interfaces (`main.rs` e `menu.rs`) chamam os mesmos métodos de `Lista`.
Nenhuma regra de negócio é duplicada: acrescentar o menu não exigiu mudar uma
linha de `tarefa.rs`.

## Desenvolvimento por etapas

O projeto não foi escrito de uma vez: cada etapa acrescentou um recurso da
linguagem, e só começou depois que a anterior rodava.

| Etapa | O que foi feito | O que ela introduziu |
| --- | --- | --- |
| 0 | Estudo dos capítulos 1 a 3 do livro oficial, com os exemplos digitados e executados: "Hello, world!", o **jogo de adivinhação** e os exercícios de tipos e controle de fluxo | Cargo, variáveis, `match`, leitura de entrada, mensagens do compilador |
| 1 | `struct Tarefa`, `enum Prioridade` e a lista em memória | tipos próprios, `Vec`, `impl` e métodos |
| 2 | Comandos `add`, `list`, `done` e `rm` | `enum Comando`, `match` exaustivo, ownership e borrowing |
| 3 | Tratamento de erros: id inexistente, prioridade inválida, comando desconhecido | `Option`, `Result` e o operador `?` |
| 4 | Gravação em `tarefas.json` | módulos, crates externas (`serde`), `std::fs` |
| 5 | Modo interativo com menu numerado | `std::io`, laço de interação, validação de entrada |
| 6 | Testes e documentação | `#[test]`, `cargo test`, `cargo fmt`, `cargo clippy` |

O histórico do Git acompanha essas etapas, com uma ressalva honesta: as etapas 1
a 4 foram desenvolvidas na mesma sessão de trabalho e entraram em um único
commit inicial; as etapas 5 e 6 têm commits próprios. Para ver:

```bash
git log --oneline
```

Antes deste projeto, o jogo de adivinhação do capítulo 2 do livro foi
implementado separadamente, como exercício. É de lá que vêm o `match` e o
tratamento de entrada do usuário que aparecem aqui.

## Uso de IA no desenvolvimento

Este projeto foi desenvolvido com apoio do Claude (Claude Code), usado como
ferramenta de ensino e de compreensão da linguagem. O uso foi o seguinte:

- **Escolha do tema e do roteiro de estudo.** A aplicação foi escolhida por
  cobrir os recursos centrais de Rust (ownership, `enum`, `match`, `Result`,
  traits, módulos, crates) em um programa pequeno o bastante para ser entendido
  por inteiro.
- **Explicação dos conceitos e das mensagens do compilador.** O borrow checker e
  o modelo de erros sem exceções foram os pontos que mais exigiram explicação.
- **Escrita do código em conjunto.** As decisões de desenho foram discutidas
  antes de serem implementadas: manter as duas interfaces (argumentos e menu),
  não adicionar a dependência `clap` nem uma biblioteca de TUI, e concentrar as
  regras de negócio em `tarefa.rs`, separadas das duas interfaces.
- **Registro no histórico do Git.** Os commits escritos com esse apoio trazem a
  linha `Co-Authored-By: Claude`, de modo que a autoria fique explícita no
  próprio repositório.

O código foi lido, executado e testado localmente, e cada recurso da linguagem
citado na tabela do início deste README aparece no código e pode ser explicado
durante a apresentação.
