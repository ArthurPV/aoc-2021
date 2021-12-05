let part_1 (arr: int array): int =
  let n = ref 0 in
  for i = 0 to Array.length arr - 2 do
    if arr.(i) < arr.(i+1) then
      n := !n+1
  done;
  !n

let part_2 (arr: int array): int =
  let n = ref 0 in
  for i = 0 to Array.length arr - 5 do
    if arr.(i) + arr.(i+1) + arr.(i+2) < arr.(i+1) + arr.(i+2) + arr.(i+3) then
      n := !n+1
  done;
  !n

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
  let arr_int = Array.map (fun s -> int_of_string s) (Array.of_list read_file) in
  Printf.printf "%d\n" (part_1 arr_int);
  Printf.printf "%d\n" (part_2 arr_int)
