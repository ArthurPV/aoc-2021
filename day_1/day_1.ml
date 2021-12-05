let rec part_1 ?(n = 0) ?(i = 0) (arr: int array): int =
  if i < Array.length arr - 1 then
    if arr.(i) < arr.(i+1) then
      part_1 ~n:(n+1) ~i:(i+1) (arr)
    else part_1 ~n:(n) ~i:(i+1) (arr)
  else n

let rec part_2 ?(n = 0) ?(i = 0) (arr: int array): int =
  if i < Array.length arr - 3 then
      if arr.(i) + arr.(i+1) + arr.(i+2) < arr.(i+1) + arr.(i+2) + arr.(i+3) then
        part_2 ~n:(n+1) ~i:(i+1) (arr)
      else part_2 ~n:(n) ~i:(i+1) (arr)
    else n

let read_file =
  let ic = open_in "input.txt" in
    let try_read () = 
      try Some (input_line ic) with End_of_file -> None in
    let rec loop acc =
      match try_read () with
      | Some s -> loop (s :: acc)
      | None -> close_in ic; List.rev acc in
  loop []

let () = 
  let arr_int = read_file
  |> Array.of_list
  |> Array.map (fun s -> int_of_string s) in
  Printf.printf "%d\n" (part_1 arr_int);
  Printf.printf "%d\n" (part_2 arr_int)
