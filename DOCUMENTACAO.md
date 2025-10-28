# 📚 Documentação do Projeto - Índice de Navegação

## 📖 Estrutura da Documentação

Este projeto contém uma documentação abrangente dividida em dois arquivos principais:

### 🚀 [README.md](./README.md) - Documentação do Projeto
**Objetivo**: Documentação geral para usuários e desenvolvedores

**Conteúdo:**
- Visão geral do projeto e características
- Instruções de instalação e execução
- Guia de uso prático
- Estrutura do código e principais funções
- Objetivos educacionais
- Aspectos de segurança implementados
- Recursos para educadores
- Exercícios sugeridos

**Público-alvo**: Estudantes, professores e qualquer pessoa interessada em usar o projeto

### 🔬 [criptografia_rsa.md](./criptografia_rsa.md) - Explicação Técnica Detalhada
**Objetivo**: Explicação profunda do algoritmo RSA

**Conteúdo:**
- Fundamentos matemáticos completos
- Algoritmo RSA passo-a-passo com provas
- Implementação detalhada de cada função
- Análise de segurança e vulnerabilidades
- Otimizações e considerações de performance
- Ataques conhecidos e contramedidas
- Comparação com outros algoritmos
- Considerações para implementações de produção

**Público-alvo**: Estudantes avançados, pesquisadores e profissionais de cibersegurança

## 🎯 Como Usar Esta Documentação

### Para uma Aula Introdutória (45 min):
1. **Leia** [README.md](./README.md) - seções "Visão Geral" e "Como Usar" (10 min)
2. **Execute** o programa com diferentes palavras (10 min)
3. **Explique** os conceitos básicos usando as seções educacionais (15 min)
4. **Discuta** as aplicações práticas e segurança (10 min)

### Para uma Aula Avançada (90 min):
1. **Revisão** dos conceitos básicos do README.md (15 min)
2. **Estudo** dos fundamentos matemáticos em [criptografia_rsa.md](./criptografia_rsa.md) (30 min)
3. **Análise** do código implementado (25 min)
4. **Discussão** sobre segurança e limitações (20 min)

### Para Estudo Individual:
1. **Comece** com [README.md](./README.md) para entender o contexto
2. **Execute** o programa várias vezes com diferentes entradas
3. **Aprofunde** com [criptografia_rsa.md](./criptografia_rsa.md)
4. **Experimente** modificações no código
5. **Compare** com implementações de produção

## 🔗 Links Rápidos por Tópico

### Conceitos Básicos
- [O que é RSA?](./criptografia_rsa.md#-introdução-ao-rsa)
- [Como executar o programa](./README.md#-como-usar)
- [Objetivos educacionais](./README.md#-objetivos-educacionais)

### Matemática
- [Fundamentos matemáticos](./criptografia_rsa.md#-fundamentos-matemáticos)
- [Algoritmo passo-a-passo](./criptografia_rsa.md#️-algoritmo-rsa-passo-a-passo)
- [Provas matemáticas](./criptografia_rsa.md#prova-matemática-por-que-funciona)

### Implementação
- [Estrutura do código](./README.md#-principais-funções)
- [Implementação detalhada](./criptografia_rsa.md#-implementação-detalhada)
- [Funções principais](./README.md#-principais-funções)

### Segurança
- [Aspectos de segurança](./README.md#-aspectos-de-segurança-abordados)
- [Análise de segurança completa](./criptografia_rsa.md#-análise-de-segurança)
- [Ataques e limitações](./criptografia_rsa.md#-limitações-e-ataques)

### Para Educadores
- [Sugestões de uso em aula](./README.md#-para-educadores)
- [Exercícios práticos](./README.md#exercícios-sugeridos)
- [Comparações com outros algoritmos](./criptografia_rsa.md#-comparação-com-outros-algoritmos)

## ⚠️ Avisos Importantes

### Este Projeto É Educacional
- **NÃO use em produção** - implementação simplificada
- **Chaves pequenas** (512 bits) apenas para demonstração
- **Sem padding seguro** - vulnerável a ataques em ambiente real
- **Foco na clareza**, não na segurança máxima

### Para Uso em Produção
- Use bibliotecas testadas (OpenSSL, ring, etc.)
- Chaves mínimo 2048 bits
- Implemente padding OAEP/PSS
- Considere proteções contra side-channel attacks

## 🛠️ Arquivos de Código

### [src/main.rs](./src/main.rs)
**Implementação completa do RSA educacional**

**Principais estruturas:**
```rust
struct ChaveRSA        // Armazena chaves RSA
```

**Principais funções:**
```rust
gerar_chaves_rsa()     // Gera par de chaves
criptografar_rsa()     // Criptografa mensagem
descriptografar_rsa()  // Descriptografa mensagem
eh_primo()             // Teste Miller-Rabin
exponenciacao_modular() // Exponenciação eficiente
```

### [Cargo.toml](./Cargo.toml)
**Configuração do projeto e dependências**

```toml
[dependencies]
num-bigint = { version = "0.4", features = ["rand"] }
rand = "0.8"
```

## 🎓 Sugestões de Uso Acadêmico

### Disciplinas Aplicáveis
- **Segurança da Informação**
- **Criptografia**
- **Matemática Computacional**
- **Teoria dos Números**
- **Programação em Rust**

### Níveis de Ensino
- **Graduação** (Ciência da Computação, Engenharia)
- **Pós-graduação** (Segurança, Criptografia)
- **Cursos Técnicos** (Cybersegurança)
- **Workshops** e **Treinamentos Profissionais**

### Extensões Possíveis
1. Implementar outros algoritmos (ECC, AES)
2. Adicionar interface gráfica
3. Comparar performance com bibliotecas profissionais
4. Implementar ataques simples (força bruta em chaves pequenas)
5. Adicionar algoritmos pós-quânticos

---

**📌 Comece sua jornada de aprendizado pelo [README.md](./README.md) e aprofunde-se com [criptografia_rsa.md](./criptografia_rsa.md)!**