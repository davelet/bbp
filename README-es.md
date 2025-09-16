# Calculadora de π con la Fórmula BBP

Este proyecto es una implementación en Rust de la fórmula Bailey-Borwein-Plouffe (BBP) para calcular los dígitos de π. Su característica más notable es la capacidad de calcular directamente el n-ésimo dígito hexadecimal de π sin necesidad de calcular todos los dígitos anteriores.

## La Fórmula BBP

La fórmula BBP fue descubierta en 1995 por Simon Plouffe en colaboración con David H. Bailey y Peter Borwein. Proporciona un método para calcular π en base 16.

La fórmula es:
![Fórmula BBP](https://latex.codecogs.com/png.latex?\pi%20=%20\sum_{k=0}^{\infty}%20\frac{1}{16^k}%20\left(%20\frac{4}{8k+1}%20-%20\frac{2}{8k+4}%20-%20\frac{1}{8k+5}%20-%20\frac{1}{8k+6}%20\right))

Este algoritmo spigot permite extraer directamente el dígito hexadecimal de π.

## Lógica Matemática: Guía para Estudiantes

Imagínate que deseas encontrar el dígito número 100 de π. Normalmente, se tendrían que calcular los 99 dígitos anteriores. La fórmula BBP actúa como un truco mágico que te permite obtener directamente el dígito hexadecimal número 100 sin necesidad de los anteriores.

### La Idea Principal: Desplazar el Punto Decimal

Por ejemplo, π en hexadecimal se escribe como `3.243F6A...`.

- El primer dígito es `2`.
- El segundo es `4`.
- El tercero es `3`.

Si deseamos encontrar el segundo dígito (`4`), multiplicamos π por 16 (que se escribe como `10` en hexadecimal), desplazando el punto decimal una posición a la derecha:
```
π * 16^1 = 32.43F6A...
```
Ahora, el dígito buscado (`4`) es el primero después del nuevo punto decimal.

Si se quiere el cuarto dígito (`F`), se multiplica por `16^3`:
```
π * 16^3 = 3243.F6A...
```
Entonces, el dígito buscado aparece en la posición correcta.

La idea básica es: **Para encontrar el n-ésimo dígito hexadecimal de π, calculamos `16^(n-1) * π` y observamos el primer dígito después del punto.**

### El Problema: No Podemos Usar π Directamente

Aunque suene sencillo, para calcular `16^(n-1) * π` se requiere conocer π, un número infinito. La computadora no puede almacenar todos sus dígitos.

Aquí es donde la fórmula BBP resulta útil, ya que permite calcular la **parte fraccionaria** de `16^(n-1) * π` sin conocer π en su totalidad.

### La Solución: Dividir la Fórmula BBP

La fórmula BBP es una suma infinita de fracciones pequeñas. Al multiplicarla por `16^(n-1)`, se obtiene:
![Fórmula BBP multiplicada](https://latex.codecogs.com/png.latex?16^{n-1}\pi%20=%20\sum_{k=0}^{\infty}%20\left(%20\frac{4\cdot%2016^{n-1-k}}{8k+1}%20-%20\frac{2\cdot%2016^{n-1-k}}{8k+4}%20-%20\frac{1\cdot%2016^{n-1-k}}{8k+5}%20-%20\frac{1\cdot%2016^{n-1-k}}{8k+6}%20\right))

Nos enfocamos en obtener la parte fraccionaria de la suma, dividiéndola en dos partes:

1. **Head (para `k` de 0 a `n-1`):** Estos términos, con exponentes positivos, generan números grandes. La contribución fraccionaria se calcula mediante exponenciación modular.
2. **Tail (para `k` de `n` a infinito):** Estos términos, con exponentes negativos, generan números muy pequeños (por ejemplo, `1/16`, `1/256`). Se suman utilizando números de punto flotante (f64).

Al sumar ambas partes, se obtiene la parte fraccionaria de `16^(n-1) * π`.

### Requisitos Previos

Para comprender esta explicación, se recomienda tener nociones básicas de:
- Representación de números en base 16,
- Series infinitas y sumatorias,
- Aritmética modular (cálculo de residuos).

### Breve Introducción a la Aritmética Modular

La aritmética modular se ocupa del cálculo de residuos. Por ejemplo, "a mod m" es el residuo de dividir a entre m. Un ejemplo útil es:
- `16^3 = 4096`, y `4096 mod 7 = 1`.
Esto permite calcular eficientemente potencias modulares sin tratar con números enormes.

### Ejemplo Numérico (n = 2)

Para obtener el segundo dígito hexadecimal (que es `4` en `3.243F6A...`), se procede de la siguiente manera:
1. Se calcula `16^(2-1) = 16` y se multiplica por π.
2. Se divide la suma en dos partes: **Head** y **Tail**.
   - **Head:** Se calcula la contribución fraccionaria mediante exponenciación modular.
   - **Tail:** Se suman los términos muy pequeños con f64.
3. La suma final, multiplicada por 16, da como resultado el dígito esperado.

### Nota sobre la Precisión del Tail

En la parte Tail se utiliza f64 ya que los términos decrecen rápidamente. Si se requiere alta precisión, se recomienda usar aritmética de precisión arbitraria (como el crate `bigdecimal` en Rust).

## Estructura del Código

El proyecto se divide en tres módulos principales:

### `src/main.rs`

- Punto de entrada del programa.
- Llama a `bbp::get_digits()` para calcular los dígitos hexadecimales de π.
- Convierte y muestra la representación decimal utilizando `decimal::hex_pi_to_decimal()`.

### `src/bbp/mod.rs`

Contiene la lógica central de la fórmula BBP:
- Funciones: `get_digit(n)`, `series_sum(n, j)` y `mod_pow()`.

### `src/decimal/mod.rs`

Se encarga de convertir los dígitos hexadecimales en una representación decimal, utilizando el crate `bigdecimal` para cálculos de alta precisión.

## Uso

### Requisitos

Se requiere instalar el toolchain de Rust (visita [rustup.rs](https://rustup.rs/)).

### Ejecución

Ejecuta:
```bash
cargo run
```
El programa mostrará:
1. Los primeros 500 dígitos hexadecimales de π.
2. La conversión a decimal.
3. Ejemplos individuales (por ejemplo, los dígitos 10 y 50).
4. Una validación con los dígitos decimales conocidos de π.

### Compilación

Para compilar un ejecutable optimizado:
```bash
cargo build --release
```
El binario se encontrará en `target/release/bbp`.

## Dependencias

- `bigdecimal`: Se utiliza para realizar cálculos de precisión arbitraria durante la conversión de hexadecimal a decimal.

## Prueba de Correctitud

La validez de la fórmula BBP se fundamenta en la expansión de π como una serie infinita. Al analizar la convergencia de la serie y la cancelación de errores entre sus términos, se demuestra que la suma converge exponencialmente hacia π. Para una demostración detallada, consulte el artículo original de Bailey, Borwein y Plouffe.
