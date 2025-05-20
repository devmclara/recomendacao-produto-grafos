# 🔎 Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## 📘 Descrição do Projeto

Este projeto, chamado **"Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore"**, foi desenvolvido como parte de um desafio técnico para melhorar a eficiência e escalabilidade da busca de produtos de uma loja virtual.

A solução é implementada em **Rust** e tem como foco:

- ✅ Indexação rápida por nome, marca e categoria  
- ✅ Buscas eficientes em catálogos com grandes volumes de dados  
- ✅ Diferentes tipos de consultas e filtros personalizados  
- ✅ Estrutura escalável para crescimento contínuo  
- ✅ Base para futuras integrações com algoritmos de recomendação e grafos

---

## ⚙️ Tecnologias Utilizadas

- **Rust** — Linguagem principal do projeto  
- **Crates padrão** (sem bibliotecas externas)  
- **GitHub Codespaces** — Ambiente de desenvolvimento utilizado

---

## 📁 Estrutura do Projeto

```
recomendacao-produto-grafos/
├── src/
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
├── .gitignore
└── README.md
```

---

## 🏗️ Arquitetura do Sistema

O sistema é dividido em módulos organizados para facilitar a manutenção e extensão do projeto:

- **Entrada e leitura de dados**  
- **Indexação e armazenamento eficiente em hash maps**  
- **Mecanismo de busca com filtros por nome, marca e categoria**  
- **Exibição de resultados ordenados**

Cada parte foi projetada para garantir modularidade e clareza na lógica do sistema.

---

## 🧠 Algoritmos e Estruturas de Dados Utilizados

- **Hash Maps (`std::collections::HashMap`)**: utilizados para indexação eficiente de produtos por diferentes atributos.
- **Iteradores e filtros**: empregados para aplicar múltiplas condições de busca simultaneamente.
- **Busca por aproximação simples**: utilizando normalização de strings (case-insensitive, remoção de acentos).

Essas estruturas garantem buscas rápidas mesmo em grandes volumes de dados.

---

## 🚀 Considerações sobre Desempenho e Escalabilidade

- O sistema foi projetado com foco em **eficiência**, minimizando o tempo de resposta das buscas.
- Utilizando tabelas hash, as buscas têm desempenho próximo de **O(1)** para operações de acesso direto.
- Testes locais com catálogos grandes (milhares de produtos simulados) demonstraram boa performance.
- A estrutura é preparada para crescer com facilidade e receber melhorias futuras (como recomendação baseada em grafos ou aprendizado de máquina).

---

## ▶️ Como Executar o Sistema de Busca

### ✅ Pré-requisitos

- [Rust instalado](https://www.rust-lang.org/tools/install)

### ✅ Passos

Clone o repositório:

```bash
git clone https://github.com/devmclara/recomendacao-produto-grafos.git
cd recomendacao-produto-grafos
```

Compile o projeto:

```bash
cargo build
```

Execute o sistema:

```bash
cargo run
```

---

## 🧪 Como Executar os Testes

Este projeto contém testes automatizados para garantir o funcionamento da lógica de busca.

Para executá-los, use o comando:

```bash
cargo test
```

Os testes serão executados automaticamente e os resultados aparecerão no terminal.

---

## 🤝 Contribuições

Contribuições são bem-vindas! Para colaborar:

1. Faça um fork do repositório
2. Crie uma branch: `git checkout -b minha-feature`
3. Commit suas alterações: `git commit -m 'Adiciona nova funcionalidade'`
4. Envie um push para sua branch: `git push origin minha-feature`
5. Abra um Pull Request

---

## 📄 Licença

Este projeto é de uso acadêmico e educacional. Para fins comerciais, entre em contato com os autores.
