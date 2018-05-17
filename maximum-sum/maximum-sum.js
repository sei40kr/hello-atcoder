const main = input => {
  const lines = input.split('\n');
  const line0 = lines[0].split(' ').map(Number);
  const a = line0[0],
    b = line0[1],
    c = line0[2];
  const k = Number(lines[1]);
  console.log(a + b + c + Math.max(a, b, c) * (Math.pow(2, k) - 1));
};

main(require('fs').readFileSync('/dev/stdin', 'utf8'));
