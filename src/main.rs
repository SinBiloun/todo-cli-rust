mod menu;
mod tarefa;

use std::env;
use std::path::PathBuf;
use std::process;
use tarefa::{Lista, Prioridade};

const ARQUIVO: &str = "tarefas.json";

#[derive(Debug, PartialEq)]
enum Comando {
    Adicionar {
        descricao: String,
        prioridade: Prioridade,
    },
    Listar {
        apenas_pendentes: bool,
    },
    Concluir(u32),
    Remover(u32),
    Menu,
    Ajuda,
}

fn main() {
    // O primeiro argumento e o caminho do proprio executavel: por isso o skip(1).
    let argumentos: Vec<String> = env::args().skip(1).collect();

    if let Err(erro) = executar(argumentos) {
        eprintln!("erro: {erro}");
        process::exit(1);
    }
}

fn executar(argumentos: Vec<String>) -> Result<(), String> {
    let comando = analisar(&argumentos)?;
    let caminho = PathBuf::from(ARQUIVO);
    let mut lista = Lista::carregar(&caminho)?;

    match comando {
        Comando::Ajuda => {
            ajuda();
            return Ok(());
        }
        // O menu grava o arquivo a cada alteracao, por isso retorna direto.
        Comando::Menu => return menu::executar(&mut lista, &caminho),
        Comando::Adicionar {
            descricao,
            prioridade,
        } => {
            let id = lista.adicionar(descricao, prioridade);
            println!("tarefa {id} adicionada");
        }
        Comando::Listar { apenas_pendentes } => {
            let tarefas = lista.listar(apenas_pendentes);
            if tarefas.is_empty() {
                println!("nenhuma tarefa");
            }
            for t in tarefas {
                println!("{t}");
            }
            return Ok(());
        }
        Comando::Concluir(id) => {
            let t = lista.concluir(id)?;
            println!("concluida: {}", t.descricao);
        }
        Comando::Remover(id) => {
            let t = lista.remover(id)?;
            println!("removida: {}", t.descricao);
        }
    }

    lista.salvar(&caminho)
}

fn analisar(argumentos: &[String]) -> Result<Comando, String> {
    // Sem argumentos: abre o menu interativo.
    let Some(comando) = argumentos.first().map(|s| s.as_str()) else {
        return Ok(Comando::Menu);
    };

    match comando {
        "add" => {
            let descricao = argumentos
                .get(1)
                .ok_or("informe a descricao: add \"minha tarefa\" [prioridade]")?
                .clone();
            let prioridade = match argumentos.get(2) {
                Some(texto) => Prioridade::analisar(texto)?,
                None => Prioridade::Media,
            };
            Ok(Comando::Adicionar {
                descricao,
                prioridade,
            })
        }
        "list" => Ok(Comando::Listar {
            apenas_pendentes: argumentos.iter().any(|a| a == "--pendentes"),
        }),
        "done" => Ok(Comando::Concluir(analisar_id(argumentos.get(1))?)),
        "rm" => Ok(Comando::Remover(analisar_id(argumentos.get(1))?)),
        "help" | "--help" | "-h" => Ok(Comando::Ajuda),
        outro => Err(format!(
            "comando desconhecido: '{outro}' (use help para ver os comandos)"
        )),
    }
}

fn analisar_id(argumento: Option<&String>) -> Result<u32, String> {
    let texto = argumento.ok_or("informe o id da tarefa")?;
    texto
        .parse()
        .map_err(|_| format!("id invalido: '{texto}' (esperado um numero inteiro)"))
}

fn ajuda() {
    println!(
        "todo_cli - gerenciador de tarefas em linha de comando

Sem argumentos, abre o menu interativo.

COMANDOS
  add <descricao> [alta|media|baixa]   adiciona uma tarefa
  list [--pendentes]                   lista as tarefas por prioridade
  done <id>                            marca a tarefa como concluida
  rm <id>                              remove a tarefa
  help                                 mostra esta ajuda

As tarefas sao gravadas em {ARQUIVO} no diretorio atual."
    );
}

#[cfg(test)]
mod testes {
    use super::*;

    fn args(valores: &[&str]) -> Vec<String> {
        valores.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn sem_argumentos_abre_o_menu() {
        assert_eq!(analisar(&args(&[])).unwrap(), Comando::Menu);
        assert_eq!(analisar(&args(&["help"])).unwrap(), Comando::Ajuda);
    }

    #[test]
    fn add_usa_prioridade_media_por_padrao() {
        let esperado = Comando::Adicionar {
            descricao: "estudar".to_string(),
            prioridade: Prioridade::Media,
        };
        assert_eq!(analisar(&args(&["add", "estudar"])).unwrap(), esperado);
    }

    #[test]
    fn id_nao_numerico_devolve_erro() {
        assert!(analisar(&args(&["done", "abc"])).is_err());
        assert!(analisar(&args(&["done"])).is_err());
    }

    #[test]
    fn comando_desconhecido_devolve_erro() {
        assert!(analisar(&args(&["voar"])).is_err());
    }
}
