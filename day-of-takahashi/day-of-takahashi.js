const main = input => {
  const line0 = input.split(' ').map(Number);
  const a = line0[0],
    b = line0[1];
  console.log(a + (a < b ? 0 : -1));
};

main(require('fs').readFileSync('/dev/stdin', 'utf8'));
