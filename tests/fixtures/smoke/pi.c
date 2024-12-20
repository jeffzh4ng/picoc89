source code
/* The Computer Language Benchmarks Game
 * https://salsa.debian.org/benchmarksgame-team/benchmarksgame/
 *
 * Contributed by Mr Ledrug
*/

#include <stdio.h>
#include <stdlib.h>
#include <gmp.h>

mpz_t tmp1, tmp2, acc, den, num;
typedef unsigned int ui;

ui extract_digit(ui nth) {
   // joggling between tmp1 and tmp2, so GMP won't have to use temp buffers
   mpz_mul_ui(tmp1, num, nth);
   mpz_add(tmp2, tmp1, acc);
   mpz_tdiv_q(tmp1, tmp2, den);

   return mpz_get_ui(tmp1);
}

void eliminate_digit(ui d) {
   mpz_submul_ui(acc, den, d);
   mpz_mul_ui(acc, acc, 10);
   mpz_mul_ui(num, num, 10);
}

void next_term(ui k) {
   ui k2 = k * 2U + 1U;

   mpz_addmul_ui(acc, num, 2U);
   mpz_mul_ui(acc, acc, k2);
   mpz_mul_ui(den, den, k2);
   mpz_mul_ui(num, num, k);
}

int main(int argc, char **argv) {
   ui d, k, i;
   int n = atoi(argv[1]);

   mpz_init(tmp1);
   mpz_init(tmp2);

   mpz_init_set_ui(acc, 0);
   mpz_init_set_ui(den, 1);
   mpz_init_set_ui(num, 1);

   for (i = k = 0; i < n;) {
      next_term(++k);
      if (mpz_cmp(num, acc) > 0)
         continue;

      d = extract_digit(3);
      if (d != extract_digit(4))
         continue;

      putchar('0' + d);
      if (++i % 10 == 0)
         printf("\t:%u\n", i);
      eliminate_digit(d);
   }

   return 0;
}