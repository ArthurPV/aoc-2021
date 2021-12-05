let part_1 (arr: string array): int =
  let h = ref 0 in
  let d = ref 0 in
  let rec loop i =
    if i < Array.length arr then
      let s = arr.(i) |> String.split_on_char ' ' in
      match s with
      | ["down"; n] ->
          let c = n |> int_of_string in
          d := !d + c;
          loop (i+1)
      | ["forward"; n] ->
          let c = n |> int_of_string in
          h := !h + c;
          loop (i+1)
      | ["up"; n] ->
          let c = n |> int_of_string in
          d := !d - c;
          loop (i+1)
      | _ -> () in
  loop (0);
  !h * !d

let part_2 (arr: string array): int =
    let h = ref 0 in
    let d = ref 0 in
    let a = ref 0 in
    let rec loop i =
      if i < Array.length arr then
        let s = arr.(i) |> String.split_on_char ' ' in
        match s with
        | ["down"; n] ->
            let c = n |> int_of_string in
            a := !a + c;
            loop (i+1)
        | ["forward"; n] ->
            let c = n |> int_of_string in
            h := !h + c;
            d := !d + (c * !a);
            loop (i+1)
        | ["up"; n] ->
            let c = n |> int_of_string in
            a := !a - c;
            loop (i+1)
        | _ -> () in
    loop (0);
    !h * !d

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
  let arr = read_file |> Array.of_list in
  Printf.printf "%d\n" (part_1 arr);
  Printf.printf "%d\n" (part_2 arr)
