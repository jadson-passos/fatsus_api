pub fn apenas_digitos(valor: &str) -> String {
    valor.chars().filter(|c| c.is_ascii_digit()).collect()
}

#[cfg(test)]
mod tests {
    use super::apenas_digitos;

    #[test]
    fn remove_pontuacao_de_cpf() {
        assert_eq!(apenas_digitos("100.000.000-01"), "10000000001");
    }

    #[test]
    fn remove_pontuacao_de_cnpj() {
        assert_eq!(apenas_digitos("11.222.333/0001-01"), "11222333000101");
    }

    #[test]
    fn ignora_texto_sem_digitos() {
        assert_eq!(apenas_digitos("abc"), "");
    }

    #[test]
    fn preserva_cnes() {
        assert_eq!(apenas_digitos("2560001"), "2560001");
    }
}
