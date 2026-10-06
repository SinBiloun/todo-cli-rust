use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::Path;

/// Prioridade de uma tarefa. A ordem da declaracao define a ordem de
/// comparacao gerada por `#[derive(Ord)]`: Alta < Media < Baixa.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Prioridade {
    Alta,
    Media,
    Baixa,
}

impl Prioridade {
    pub fn analisar(texto: &str) -> Result<Prioridade, String> {
        match texto.to_lowercase().as_str() {
            "alta" => Ok(Prioridade::Alta),
            "media" | "média" => Ok(Prioridade::Media),
            "baixa" => Ok(Prioridade::Baixa),
            outro => Err(format!(
                "prioridade invalida: '{outro}' (use alta, media ou baixa)"
            )),
        }
    }
}

impl fmt::Display for Prioridade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let texto = match self {
            Prioridade::Alta => "alta",
            Prioridade::Media => "media",
            Prioridade::Baixa => "baixa",
        };
        write!(f, "{texto}")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tarefa {
    pub id: u32,
    pub descricao: String,
    pub prioridade: Prioridade,
    pub concluida: bool,
}

impl fmt::Display for Tarefa {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let marca = if self.concluida { "x" } else { " " };
        write!(
            f,
            "[{}] {:>3}  {:<40} ({})",
            marca, self.id, self.descricao, self.prioridade
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Lista {
    tarefas: Vec<Tarefa>,
}

impl Lista {
    /// Le a lista do disco. Arquivo inexistente nao e erro: significa
    /// que ainda nao existe nenhuma tarefa.
    pub fn carregar(caminho: &Path) -> Result<Lista, String> {
        if !caminho.exists() {
            return Ok(Lista::default());
        }
        let conteudo =
            fs::read_to_string(caminho).map_err(|e| format!("falha ao ler {caminho:?}: {e}"))?;
        serde_json::from_str(&conteudo).map_err(|e| format!("arquivo de dados invalido: {e}"))
    }

    pub fn salvar(&self, caminho: &Path) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(caminho, json).map_err(|e| format!("falha ao gravar {caminho:?}: {e}"))
    }

    pub fn adicionar(&mut self, descricao: String, prioridade: Prioridade) -> u32 {
        let id = self.tarefas.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        self.tarefas.push(Tarefa {
            id,
            descricao,
            prioridade,
            concluida: false,
        });
        id
    }

    pub fn concluir(&mut self, id: u32) -> Result<&Tarefa, String> {
        let tarefa = self.buscar_mut(id)?;
        tarefa.concluida = true;
        Ok(tarefa)
    }

    pub fn remover(&mut self, id: u32) -> Result<Tarefa, String> {
        let posicao = self
            .tarefas
            .iter()
            .position(|t| t.id == id)
            .ok_or_else(|| format!("tarefa {id} nao encontrada"))?;
        Ok(self.tarefas.remove(posicao))
    }

    /// Tarefas ordenadas por prioridade; `apenas_pendentes` esconde as concluidas.
    pub fn listar(&self, apenas_pendentes: bool) -> Vec<&Tarefa> {
        let mut visiveis: Vec<&Tarefa> = self
            .tarefas
            .iter()
            .filter(|t| !apenas_pendentes || !t.concluida)
            .collect();
        visiveis.sort_by_key(|t| (t.concluida, t.prioridade));
        visiveis
    }

    fn buscar_mut(&mut self, id: u32) -> Result<&mut Tarefa, String> {
        self.tarefas
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or_else(|| format!("tarefa {id} nao encontrada"))
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    fn lista_exemplo() -> Lista {
        let mut lista = Lista::default();
        lista.adicionar("estudar ownership".to_string(), Prioridade::Baixa);
        lista.adicionar("entregar projeto".to_string(), Prioridade::Alta);
        lista
    }

    #[test]
    fn ids_sao_sequenciais_e_concluir_marca_a_tarefa() {
        let mut lista = lista_exemplo();
        assert_eq!(lista.concluir(1).unwrap().id, 1);
        assert!(lista.listar(false)[1].concluida);
        assert_eq!(lista.listar(true).len(), 1);
    }

    #[test]
    fn id_inexistente_devolve_erro_em_vez_de_panico() {
        let mut lista = lista_exemplo();
        assert!(lista.remover(99).is_err());
        assert!(lista.concluir(99).is_err());
    }

    #[test]
    fn listagem_ordena_por_prioridade() {
        let lista = lista_exemplo();
        assert_eq!(lista.listar(false)[0].descricao, "entregar projeto");
    }

    #[test]
    fn prioridade_invalida_devolve_erro() {
        assert_eq!(Prioridade::analisar("ALTA"), Ok(Prioridade::Alta));
        assert!(Prioridade::analisar("urgente").is_err());
    }
}
