#[expect(
    clippy::manual_swap,
    reason = "as trocas são escritas à mão de propósito — entender a mecânica \
              da troca é objetivo do repo, não usar o helper da std"
)]
pub fn quick_sort(arr: &mut [i32]) {
    
    let n = arr.len();

    if n <= 1 {
        return;
    }

    let aux = arr[n - 1];

    arr[n - 1] = arr[n/2];

    arr[n/2] = aux; 

    let pivo = arr[n - 1];

    let mut i = 0;

    for j in 0..n - 1 {

        if arr[j] <= pivo {
            let aux = arr[i];

            arr[i] = arr[j];

            arr[j] = aux;
            i += 1;
        } 
    }

    let aux = arr[n - 1];

    arr[n - 1] = arr[i];
    arr[i] = aux;

    let (esq, dir) = arr.split_at_mut(i);

    quick_sort(esq);
    quick_sort(&mut dir[1..]);






}

#[cfg(test)]
mod tests {
    use super::*;

    /// Encurta vetores longos para o print não virar uma parede de números.
    fn resumo(v: &[i32]) -> String {
        const MAX: usize = 12;

        if v.len() <= MAX {
            format!("{v:?}")
        } else {
            format!("{:?} ... (+{})", &v[..MAX], v.len() - MAX)
        }
    }

    /// Ordena a mesma entrada com a std e compara. Pega ordem errada e também
    /// elemento perdido ou duplicado pelas trocas do particionamento.
    /// Imprime antes/depois — visível com `cargo test -- --show-output`.
    fn verifica(entrada: Vec<i32>) {
        let mut esperado = entrada.clone();
        esperado.sort();

        let mut obtido = entrada.clone();
        quick_sort(&mut obtido);

        println!("  antes:  {}", resumo(&entrada));
        println!("  depois: {}", resumo(&obtido));

        assert_eq!(obtido, esperado, "falhou para a entrada {entrada:?}");
    }

    #[test]
    fn vazio() {
        verifica(vec![]);
    }

    #[test]
    fn um_elemento() {
        verifica(vec![7]);
    }

    #[test]
    fn dois_fora_de_ordem() {
        verifica(vec![2, 1]);
    }

    #[test]
    fn desordenado() {
        verifica(vec![8, 2, 9, 1, 5]);
    }

    #[test]
    fn ja_ordenado() {
        verifica(vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn ordem_inversa() {
        verifica(vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn com_duplicatas() {
        verifica(vec![3, 1, 3, 2, 3]);
    }

    #[test]
    fn todos_iguais() {
        verifica(vec![4, 4, 4, 4]);
    }

    #[test]
    fn com_negativos() {
        verifica(vec![-5, 2, -9, 0, 4]);
    }

    #[test]
    fn extremos_do_i32() {
        verifica(vec![i32::MAX, 0, i32::MIN]);
    }

    /// O pivô (último elemento) é o menor de todos: a partição sai vazia de um
    /// lado, o caso mais desbalanceado possível.
    #[test]
    fn pivo_e_o_menor() {
        verifica(vec![9, 7, 8, 6, 1]);
    }

    /// O espelho: o pivô é o maior, e todo o resto vai pra esquerda.
    #[test]
    fn pivo_e_o_maior() {
        verifica(vec![3, 1, 4, 2, 9]);
    }

    /// A assinatura recebe `&mut [i32]`, então array de tamanho fixo também
    /// serve — não só `Vec`.
    #[test]
    fn aceita_array_fixo() {
        let mut a = [3, 1, 2];

        println!("  antes:  {}", resumo(&a));
        quick_sort(&mut a);
        println!("  depois: {}", resumo(&a));

        assert_eq!(a, [1, 2, 3]);
    }

    /// Entrada grande e bagunçada, gerada por um LCG para ser sempre a mesma.
    /// Pivô aleatório na prática, então a recursão fica balanceada.
    #[test]
    fn muitos_elementos() {
        let mut estado: u32 = 12345;
        let mut v = Vec::with_capacity(500);

        for _ in 0..500 {
            estado = estado.wrapping_mul(1664525).wrapping_add(1013904223);
            v.push((estado >> 16) as i32 - 16384);
        }

        verifica(v);
    }

    /// Pior caso do pivô ingênuo: em ordem inversa, o último elemento é sempre
    /// o menor, então cada partição descasca um elemento só e a recursão fica
    /// com profundidade n. Mantido pequeno de propósito — ver o comentário no
    /// teste `entrada_grande`.
    #[test]
    fn pior_caso_pequeno() {
        verifica((0..500).rev().collect());
    }

    /// Entrada grande, embaralhada. NÃO em ordem inversa: com pivô no último
    /// elemento isso daria profundidade de recursão igual a n e estouraria a
    /// pilha. Quando o pivô virar o do meio, dá pra trocar por
    /// `(0..10_000).rev()` como no merge_sort.
    #[test]
    fn entrada_grande() {
        let mut estado: u32 = 999;
        let mut v = Vec::with_capacity(10_000);

        for _ in 0..10_000 {
            estado = estado.wrapping_mul(1664525).wrapping_add(1013904223);
            v.push((estado >> 8) as i32);
        }

        verifica(v);
    }
}
