let s = (System.IO.File.ReadAllText "../input/06").Trim().Split '\t' |> Seq.toList
let l = s |> Seq.map System.Int32.Parse |> Seq.toList

let nextList cur = 0
  
  

let partOne c m l : int = 
  if m.ContainsKey(l) then m.[l]
  else
    0

printfn "Part one: %d" partOne 0 Map.empty.Add(l) l




