use num_bigint::{BigInt, ToBigInt, RandBigInt};
use std::io;

/// Estrutura para armazenar as chaves RSA
#[derive(Debug, Clone)]
struct ChaveRSA {
    n: BigInt,      // Módulo (p * q)
    e: BigInt,      // Expoente público
    d: BigInt,      // Expoente privado
}

/// Implementa o algoritmo estendido de Euclides para encontrar o inverso modular
fn algoritmo_euclidiano_estendido(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if *a == 0.to_bigint().unwrap() {
        return (b.clone(), 0.to_bigint().unwrap(), 1.to_bigint().unwrap());
    }
    
    let (gcd, x1, y1) = algoritmo_euclidiano_estendido(&(b % a), a);
    let x = &y1 - (b / a) * &x1;
    let y = x1;
    
    (gcd, x, y)
}

/// Calcula o inverso modular de 'a' módulo 'm'
fn inverso_modular(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let (gcd, x, _) = algoritmo_euclidiano_estendido(a, m);
    
    if gcd != 1.to_bigint().unwrap() {
        None // Não existe inverso modular
    } else {
        Some(((x % m) + m) % m)
    }
}

/// Teste de primalidade de Miller-Rabin (versão simplificada)
fn eh_primo(n: &BigInt, k: u32) -> bool {
    if *n <= 1.to_bigint().unwrap() {
        return false;
    }
    if *n <= 3.to_bigint().unwrap() {
        return true;
    }
    if n % 2 == 0.to_bigint().unwrap() {
        return false;
    }

    // Escrever n-1 como d * 2^r
    let mut r = 0;
    let mut d = n - 1;
    while &d % 2 == 0.to_bigint().unwrap() {
        d /= 2;
        r += 1;
    }

    'witness: for _ in 0..k {
        let mut rng = rand::thread_rng();
        let a = rng.gen_bigint_range(&2.to_bigint().unwrap(), &(n - 1));
        let mut x = exponenciacao_modular(&a, &d, n);

        if x == 1.to_bigint().unwrap() || x == n - 1 {
            continue 'witness;
        }

        for _ in 0..r - 1 {
            x = exponenciacao_modular(&x, &2.to_bigint().unwrap(), n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

/// Exponenciação modular rápida: (base^exp) mod modulo
fn exponenciacao_modular(base: &BigInt, exp: &BigInt, modulo: &BigInt) -> BigInt {
    if *modulo == 1.to_bigint().unwrap() {
        return 0.to_bigint().unwrap();
    }
    
    let mut resultado = 1.to_bigint().unwrap();
    let mut base = base % modulo;
    let mut exp = exp.clone();
    
    while exp > 0.to_bigint().unwrap() {
        if &exp % 2 == 1.to_bigint().unwrap() {
            resultado = (resultado * &base) % modulo;
        }
        exp >>= 1;
        base = (&base * &base) % modulo;
    }
    
    resultado
}

/// Gera um número primo de n bits
fn gerar_primo(bits: u32) -> BigInt {
    let mut rng = rand::thread_rng();
    
    loop {
        // Gera um número aleatório ímpar de n bits
        let mut candidato = rng.gen_bigint(bits as u64);
        
        // Garantir que seja ímpar
        if &candidato % 2 == 0.to_bigint().unwrap() {
            candidato += 1;
        }
        
        // Garantir que tenha exatamente n bits (MSB = 1)
        candidato |= 1.to_bigint().unwrap() << (bits - 1);
        
        if eh_primo(&candidato, 10) {
            return candidato;
        }
    }
}

/// Gera um par de chaves RSA
fn gerar_chaves_rsa(bits: u32) -> ChaveRSA {
    println!("🔑 Gerando chaves RSA de {} bits...", bits);
    
    // Passo 1: Gerar dois números primos p e q
    println!("   Passo 1: Gerando números primos p e q...");
    let p = gerar_primo(bits / 2);
    let q = gerar_primo(bits / 2);
    println!("   ✓ p gerado: {} bits", p.bits());
    println!("   ✓ q gerado: {} bits", q.bits());
    
    // Passo 2: Calcular n = p * q
    let n = &p * &q;
    println!("   Passo 2: Calculando n = p × q...");
    println!("   ✓ n = {} (módulo público)", n);
    
    // Passo 3: Calcular φ(n) = (p-1) * (q-1)
    let phi_n = (&p - 1) * (&q - 1);
    println!("   Passo 3: Calculando φ(n) = (p-1) × (q-1)...");
    println!("   ✓ φ(n) = {}", phi_n);
    
    // Passo 4: Escolher e (geralmente 65537)
    let e = 65537.to_bigint().unwrap();
    println!("   Passo 4: Escolhendo expoente público e = 65537");
    
    // Passo 5: Calcular d (inverso modular de e mod φ(n))
    println!("   Passo 5: Calculando expoente privado d...");
    let d = inverso_modular(&e, &phi_n).expect("Erro ao calcular inverso modular");
    println!("   ✓ d calculado com sucesso");
    
    println!("✅ Chaves RSA geradas com sucesso!\n");
    
    ChaveRSA { n, e, d }
}

/// Converte uma string em um vetor de números BigInt
fn string_para_numeros(texto: &str) -> Vec<BigInt> {
    texto
        .bytes()
        .map(|byte| byte.to_bigint().unwrap())
        .collect()
}

/// Converte um vetor de números BigInt em uma string
fn numeros_para_string(numeros: &[BigInt]) -> String {
    numeros
        .iter()
        .map(|num| num.to_bytes_be().1[0] as char)
        .collect()
}

/// Criptografa uma mensagem usando RSA
fn criptografar_rsa(mensagem: &str, chave: &ChaveRSA) -> Vec<BigInt> {
    println!("🔒 Criptografando mensagem: \"{}\"", mensagem);
    println!("   Convertendo cada caractere em número e criptografando...");
    
    let numeros = string_para_numeros(mensagem);
    let mut resultado = Vec::new();
    
    for num in numeros.iter() {
        let char_original = num.to_bytes_be().1[0] as char;
        let criptografado = exponenciacao_modular(num, &chave.e, &chave.n);
        println!("   '{}' ({}) → {}", char_original, num, criptografado);
        resultado.push(criptografado);
    }
    
    println!("✅ Criptografia concluída!\n");
    resultado
}

/// Descriptografa uma mensagem usando RSA
fn descriptografar_rsa(mensagem_criptografada: &[BigInt], chave: &ChaveRSA) -> String {
    println!("🔓 Descriptografando mensagem...");
    
    let mut numeros_descriptografados = Vec::new();
    
    for num_cripto in mensagem_criptografada.iter() {
        let descriptografado = exponenciacao_modular(num_cripto, &chave.d, &chave.n);
        let char_descriptografado = descriptografado.to_bytes_be().1[0] as char;
        println!("   {} → {} ('{}')", num_cripto, descriptografado, char_descriptografado);
        numeros_descriptografados.push(descriptografado);
    }
    
    let resultado = numeros_para_string(&numeros_descriptografados);
    println!("✅ Descriptografia concluída: \"{}\"\n", resultado);
    resultado
}

fn main() {
    println!("🎓 DEMONSTRAÇÃO DE CRIPTOGRAFIA RSA");
    println!("=====================================\n");
    
    // Solicitar entrada do usuário
    println!("Digite uma palavra para criptografar:");
    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Falha ao ler entrada");
    
    let palavra = entrada.trim();
    
    if palavra.is_empty() {
        println!("❌ Palavra vazia! Usando exemplo padrão: 'HELLO'");
        let palavra = "HELLO";
        demonstrar_rsa(palavra);
    } else {
        demonstrar_rsa(palavra);
    }
}

fn demonstrar_rsa(palavra: &str) {
    println!("📝 Palavra a ser criptografada: \"{}\"\n", palavra);
    
    // Gerar chaves RSA (usando 512 bits para demonstração - em produção use 2048+ bits)
    let chaves = gerar_chaves_rsa(512);
    
    // Mostrar informações das chaves
    println!("📋 INFORMAÇÕES DAS CHAVES:");
    println!("   Chave Pública (n, e):");
    println!("   - n (módulo): {} ({} bits)", chaves.n, chaves.n.bits());
    println!("   - e (expoente): {}", chaves.e);
    println!("   Chave Privada (n, d):");
    println!("   - n (módulo): [mesmo da chave pública]");
    println!("   - d (expoente): {} ({} bits)\n", chaves.d, chaves.d.bits());
    
    // Criptografar
    let mensagem_criptografada = criptografar_rsa(palavra, &chaves);
    
    // Mostrar resultado criptografado
    println!("📊 RESULTADO DA CRIPTOGRAFIA:");
    for (i, num) in mensagem_criptografada.iter().enumerate() {
        println!("   Posição {}: {}", i + 1, num);
    }
    println!();
    
    // Descriptografar
    let mensagem_descriptografada = descriptografar_rsa(&mensagem_criptografada, &chaves);
    
    // Verificação
    println!("🔍 VERIFICAÇÃO:");
    println!("   Mensagem original:      \"{}\"", palavra);
    println!("   Mensagem descriptografada: \"{}\"", mensagem_descriptografada);
    
    if palavra == mensagem_descriptografada {
        println!("   ✅ Sucesso! As mensagens são idênticas!");
    } else {
        println!("   ❌ Erro! As mensagens são diferentes!");
    }
    
    println!("\n📚 CONCEITOS IMPORTANTES:");
    println!("   • RSA é um algoritmo de criptografia assimétrica");
    println!("   • Usa um par de chaves: pública (para criptografar) e privada (para descriptografar)");
    println!("   • Segurança baseada na dificuldade de fatorar números grandes");
    println!("   • Cada caractere é convertido em número e processado individualmente");
    println!("   • Em produção, use chaves de pelo menos 2048 bits!");
}
