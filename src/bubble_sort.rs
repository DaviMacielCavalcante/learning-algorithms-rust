#[allow(clippy::manual_swap)]
pub fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();

    if n == 0 {
        return;
    }

    for i in 0..n - 1 {
        let mut swapped = false;
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                let aux = arr[j + 1];

                arr[j + 1] = arr[j];
                arr[j] = aux;

                swapped = true;
            }
        }
        if !swapped {
            break;
        }
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

    /// Ordena a mesma entrada com a std e compara. Pega dois tipos de erro de
    /// uma vez: ordem errada, e elemento perdido ou duplicado pela troca.
    /// Imprime antes/depois — visível com `cargo test -- --show-output`.
    fn verifica(entrada: Vec<i32>) {
        let mut esperado = entrada.clone();
        esperado.sort();

        let mut obtido = entrada.clone();
        bubble_sort(&mut obtido);

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
        verifica(vec![5, 1, 4, 2]);
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

    /// Tartaruga: o menor elemento está na última posição e precisa atravessar
    /// o array inteiro, uma casa por passada. Quebra se o laço interno não
    /// começar do zero.
    #[test]
    fn menor_elemento_no_fim() {
        verifica(vec![10, 20, 30, 40, 5]);
    }

    /// O espelho do anterior: o maior está na frente e sobe de carona numa
    /// passada só.
    #[test]
    fn maior_elemento_no_inicio() {
        verifica(vec![50, 1, 2, 3, 4]);
    }

    #[test]
    fn quase_ordenado() {
        verifica(vec![2, 3, 4, 1]);
    }

    /// A assinatura recebe `&mut [i32]`, então array de tamanho fixo também
    /// serve — não só `Vec`.
    #[test]
    fn aceita_array_fixo() {
        let mut a = [3, 1, 2];

        println!("  antes:  {}", resumo(&a));
        bubble_sort(&mut a);
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
}
