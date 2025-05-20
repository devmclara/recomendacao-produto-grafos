use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Produto {
    id: u32,
    nome: String,
    marca: String,
    categoria: String,
}

struct SistemaBusca {
    produtos: Vec<Produto>,
    indice_nome: HashMap<String, Vec<u32>>,
    indice_marca: HashMap<String, Vec<u32>>,
    indice_categoria: HashMap<String, Vec<u32>>,
}

impl SistemaBusca {
    fn new() -> Self {
        SistemaBusca {
            produtos: Vec::new(),
            indice_nome: HashMap::new(),
            indice_marca: HashMap::new(),
            indice_categoria: HashMap::new(),
        }
    }

    fn adicionar_produto(&mut self, produto: Produto) {
        let id = produto.id;
        let nome = produto.nome.to_lowercase();
        let marca = produto.marca.to_lowercase();
        let categoria = produto.categoria.to_lowercase();

        self.produtos.push(produto);

        self.indice_nome.entry(nome).or_default().push(id);
        self.indice_marca.entry(marca).or_default().push(id);
        self.indice_categoria.entry(categoria).or_default().push(id);
    }

    fn buscar(&self, termo: &str) -> Vec<&Produto> {
        let termo = termo.to_lowercase();

        let mut ids_encontrados = Vec::new();

        if let Some(ids) = self.indice_nome.get(&termo) {
            ids_encontrados.extend(ids);
        }
        if let Some(ids) = self.indice_marca.get(&termo) {
            ids_encontrados.extend(ids);
        }
        if let Some(ids) = self.indice_categoria.get(&termo) {
            ids_encontrados.extend(ids);
        }

        // Remove duplicatas
        ids_encontrados.sort_unstable();
        ids_encontrados.dedup();

        // Retorna os produtos encontrados
        ids_encontrados
            .iter()
            .filter_map(|&id| self.produtos.iter().find(|p| p.id == id))
            .collect()
    }
}

fn main() {
    let mut sistema = SistemaBusca::new();

    sistema.adicionar_produto(Produto {
        id: 1,
        nome: "Notebook Gamer".to_string(),
        marca: "Acer".to_string(),
        categoria: "Eletrônicos".to_string(),
    });

    sistema.adicionar_produto(Produto {
        id: 2,
        nome: "Smartphone Galaxy".to_string(),
        marca: "Samsung".to_string(),
        categoria: "Celulares".to_string(),
    });

    sistema.adicionar_produto(Produto {
        id: 3,
        nome: "Mouse sem fio".to_string(),
        marca: "Logitech".to_string(),
        categoria: "Periféricos".to_string(),
    });

    let termo_busca = "logitech";
    println!("Buscando por: '{}'", termo_busca);
    let resultados = sistema.buscar(termo_busca);

    if resultados.is_empty() {
        println!("Nenhum produto encontrado.");
    } else {
        for produto in resultados {
            println!(
                "ID: {}, Nome: {}, Marca: {}, Categoria: {}",
                produto.id, produto.nome, produto.marca, produto.categoria
            );
        }
    }
}
