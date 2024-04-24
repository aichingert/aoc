let input = 
  (System.IO.File.ReadAllText "../input/06").Trim().Split '\t' 
  |> Seq.toList

let parsed = 
  input
  |> Seq.map System.Int32.Parse 
  |> Seq.toList

let step (l: List<int>) : List<int> =
  let _, max, idx = 
    l |> List.fold (fun (index, maxSoFar, idx) v -> 
      if v > maxSoFar then (index+1, v, index+1)
      else (index+1, maxSoFar, idx)) (-1, System.Int32.MinValue, -1)

  let diff = ceil ((float max) / (float l.Length)) |> int
  let mutable cur = l[idx]

  printf "%A\n" l

  [0;0]

printfn "Part one: %A\n" (step parsed)
