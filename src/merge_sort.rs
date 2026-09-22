#[expect(clippy::needless_range_loop, clippy::manual_memcpy)]
pub fn merge_sort(arr: &mut [i32]) {

    let n = arr.len();

    if n <= 1 {
        return;
    }        

    let mid = n / 2;

    let (esq, dir) = arr.split_at_mut(mid);

    merge_sort(esq);
    merge_sort(dir);

    let mut buffer: Vec<i32> = Vec::with_capacity(n);

    let mut i_esq = 0;
    let mut i_dir = 0;

    let len_esq = esq.len();
    let len_dir = dir.len();


    while i_esq < len_esq && i_dir < len_dir {

        if esq[i_esq] < dir[i_dir] {
            buffer.push(esq[i_esq]);
            i_esq += 1;
        } else {
            buffer.push(dir[i_dir]);
            i_dir += 1;  
        }

    }
  
    for j in i_dir..len_dir {
        buffer.push(dir[j]);
    }

    for j in i_esq..len_esq {
        buffer.push(esq[j]);
    }
    
    for k in 0..n {
        arr[k] = buffer[k]
    }

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
    /// elemento perdido ou duplicado — o defeito clássico de uma intercalação
    /// que esquece de despejar o resto de um dos lados.
    /// Imprime antes/depois — visível com `cargo test -- --show-output`.
    fn verifica(entrada: Vec<i32>) {
        let mut esperado = entrada.clone();
        esperado.sort();

        let mut obtido = entrada.clone();
        merge_sort(&mut obtido);

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
        verifica(vec![5, 1, 4, 2, 8]);
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

    /// Divisão desigual: com n ímpar as metades têm tamanhos diferentes, e a
    /// intercalação precisa lidar com um lado esgotando antes do outro.
    #[test]
    fn tamanho_impar() {
        verifica(vec![9, 3, 7, 1, 8, 2, 5]);
    }

    /// Árvore de recursão perfeitamente balanceada — todas as metades pares.
    #[test]
    fn potencia_de_dois() {
        verifica(vec![8, 3, 5, 1, 7, 2, 6, 4]);
    }

    /// Todos os elementos de um lado são menores que os do outro: a
    /// intercalação drena a esquerda inteira antes de tocar na direita.
    #[test]
    fn metades_disjuntas() {
        verifica(vec![1, 2, 3, 100, 200, 300]);
    }

    /// O espelho do anterior, forçando o despejo do outro lado.
    #[test]
    fn metades_disjuntas_invertidas() {
        verifica(vec![100, 200, 300, 1, 2, 3]);
    }

    /// A assinatura recebe `&mut [i32]`, então array de tamanho fixo também
    /// serve — não só `Vec`.
    #[test]
    fn aceita_array_fixo() {
        let mut a = [3, 1, 2];

        println!("  antes:  {}", resumo(&a));
        merge_sort(&mut a);
        println!("  depois: {}", resumo(&a));

        assert_eq!(a, [1, 2, 3]);
    }

    /// Entrada grande e bagunçada, gerada por um LCG para ser sempre a mesma.
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

    /// 10 mil elementos em ordem inversa: confirma que a recursão aguenta a
    /// profundidade (~14 níveis) e que o custo não explode.
    #[test]
    fn entrada_grande() {
        verifica((0..10_000).rev().collect());
    }
}
