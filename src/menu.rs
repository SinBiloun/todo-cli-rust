use crate::tarefa::{Lista, Prioridade};
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Opcao {
    Adicionar,
    Listar,
    Concluir,
    Remover,
    Sair,
    Invalida(String),
}

fn interpretar(texto: &str) -> Opcao {
    match texto.trim() {
        "1" => Opcao::Adicionar,
        "2" => Opcao::Listar,
        "3" => Opcao::Concluir,
        "4" => Opcao::Remover,
        "0" | "sair" => Opcao::Sair,
        outro => Opcao::Invalida(outro.to_string()),
    }
}

/// Laco principal do modo interativo. Erros do usuario (opcao inexistente,
/// id invalido) sao avisados e o laco continua; so falhas de entrada e saida
/// viram Err e encerram o programa.
pub fn executar(lista: &mut Lista, caminho: &Path) -> Result<(), String> {
    println!("todo_cli - modo interativo (digite 0 para sair)");

    loop {
        mostrar_menu();

        let Some(entrada) = perguntar("opcao: ")? else {
            break; // fim da entrada: Ctrl+Z no Windows, Ctrl+D no Linux
        };

        let alterou = match interpretar(&entrada) {
            Opcao::Sair => break,
            Opcao::Adicionar => adicionar(lista)?,
            Opcao::Listar => {
                listar(lista);
                false
            }
            Opcao::Concluir => concluir(lista)?,
            Opcao::Remover => remover(lista)?,
            Opcao::Invalida(texto) => {
                println!("opcao invalida: '{texto}'");
                false
            }
        };

        // Grava a cada alteracao para nao perder tarefas se o terminal fechar.
        if alterou {
            lista.salvar(caminho)?;
        }
    }

    println!("ate logo");
    Ok(())
}

fn mostrar_menu() {
    println!(
        "
=== TODO CLI ===
1  Adicionar tarefa
2  Listar tarefas
3  Concluir tarefa
4  Remover tarefa
0  Sair"
    );
}

fn adicionar(lista: &mut Lista) -> Result<bool, String> {
    let Some(descricao) = perguntar("descricao: ")? else {
        return Ok(false);
    };
    if descricao.is_empty() {
        println!("descricao vazia: nada foi adicionado");
        return Ok(false);
    }

    let Some(texto) = perguntar("prioridade [alta/media/baixa] (enter = media): ")? else {
        return Ok(false);
    };
    let prioridade = if texto.is_empty() {
        Prioridade::Media
    } else {
        match Prioridade::analisar(&texto) {
            Ok(p) => p,
            Err(erro) => {
                println!("{erro}");
                return Ok(false);
            }
        }
    };

    let id = lista.adicionar(descricao, prioridade);
    println!("tarefa {id} adicionada");
    Ok(true)
}

fn listar(lista: &Lista) {
    let tarefas = lista.listar(false);
    if tarefas.is_empty() {
        println!("nenhuma tarefa");
    }
    for t in tarefas {
        println!("{t}");
    }
}

fn concluir(lista: &mut Lista) -> Result<bool, String> {
    let Some(id) = pedir_id(lista)? else {
        return Ok(false);
    };
    match lista.concluir(id) {
        Ok(t) => {
            println!("concluida: {}", t.descricao);
            Ok(true)
        }
        Err(erro) => {
            println!("{erro}");
            Ok(false)
        }
    }
}

fn remover(lista: &mut Lista) -> Result<bool, String> {
    let Some(id) = pedir_id(lista)? else {
        return Ok(false);
    };
    match lista.remover(id) {
        Ok(t) => {
            println!("removida: {}", t.descricao);
            Ok(true)
        }
        Err(erro) => {
            println!("{erro}");
            Ok(false)
        }
    }
}

/// Mostra as tarefas antes de pedir o id: sem isso o usuario teria que
/// decorar os numeros entre uma opcao e outra.
fn pedir_id(lista: &Lista) -> Result<Option<u32>, String> {
    listar(lista);
    let Some(texto) = perguntar("id: ")? else {
        return Ok(None);
    };
    match texto.parse() {
        Ok(id) => Ok(Some(id)),
        Err(_) => {
            println!("id invalido: '{texto}' (esperado um numero inteiro)");
            Ok(None)
        }
    }
}

/// Devolve None quando a entrada termina (Ctrl+Z / Ctrl+D).
fn perguntar(rotulo: &str) -> Result<Option<String>, String> {
    print!("{rotulo}");
    // Sem o flush o rotulo so apareceria depois da digitacao, porque a saida
    // padrao e armazenada em buffer ate a quebra de linha.
    io::stdout().flush().map_err(|e| e.to_string())?;

    let mut entrada = String::new();
    let lidos = io::stdin()
        .read_line(&mut entrada)
        .map_err(|e| format!("falha ao ler a entrada: {e}"))?;

    if lidos == 0 {
        return Ok(None);
    }
    Ok(Some(entrada.trim().to_string()))
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn numeros_do_menu_viram_opcoes() {
        assert_eq!(interpretar("1"), Opcao::Adicionar);
        assert_eq!(interpretar(" 2 "), Opcao::Listar);
        assert_eq!(interpretar("3"), Opcao::Concluir);
        assert_eq!(interpretar("4"), Opcao::Remover);
        assert_eq!(interpretar("0"), Opcao::Sair);
    }

    #[test]
    fn entrada_fora_do_menu_e_invalida() {
        assert_eq!(interpretar("9"), Opcao::Invalida("9".to_string()));
        assert_eq!(interpretar("abc"), Opcao::Invalida("abc".to_string()));
    }
}
