in$ = input
part_one = 0

for row = 0 to 127
  hr$ = in$ & "-" & row
  l[] = strbytes hr$ 
  arrbase l[] 0

  n[] = [ ]
  arrbase n[] 0
  len n[] 256

  for i range0 256
    n[i] = i
  .

  for seq in [ 17 31 73 47 23 ]
    l[] &= seq
  .

  posi = 0
  skip = 0

  for r range0 64
    for le in l[]
      a = posi
      b = (posi + le - 1) mod 256
      for _ range0 le div 2
        swap n[a] n[b]
        a = (a + 1) mod 256
        b = (b - 1) mod 256
      .
      posi = (posi + le + skip) mod 256
      skip += 1
    .
  .

  bin$ = ""

  for i = 0 to 15
    offset = i * 16
    hvalue = n[offset]
    for j to 15
      hvalue = bitxor hvalue n[offset + j]
    .

    cur$ = ""

    while hvalue > 0
      if hvalue mod 2 = 0
        cur$ = 0 & cur$
      else
        part_one += 1
        cur$ = 1 & cur$
      .

      hvalue = hvalue div 2
    .

    while len cur$ < 8
      cur$ = 0 & cur$
    .
  
    bin$ = bin$ & cur$
  .
.

print "Part one: " & part_one

input_data
flqrgnkx
