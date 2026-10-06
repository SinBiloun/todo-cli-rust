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
| Módulos | `src/tarefa.rs` separado de `src/main.rs` |
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

São 8 testes cobrindo a interpretação dos argumentos (comando desconhecido, id não
numérico, prioridade inválida) e as operações da lista (ids sequenciais, conclusão,
remoção de id inexistente, ordenação por prioridade).

## Estrutura

```
src/
  main.rs     interface de linha de comando: argumentos, despacho, saída
  tarefa.rs   regras de negócio: Tarefa, Prioridade, Lista e persistência
```
