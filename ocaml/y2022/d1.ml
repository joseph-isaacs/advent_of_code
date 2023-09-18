open Str;;
open Format;;
open List;;

#load "str.cma";;


let file = "../input/y2022/d1.txt";;

let read_file_into_string filename =
  let in_channel = open_in filename in
  let file_length = in_channel_length in_channel in
  let buffer = Buffer.create file_length in
  try
    while true do
      let line = input_line in_channel in
      Buffer.add_string buffer (line ^ "\n")
    done;
    Buffer.contents buffer
  with End_of_file ->
    close_in in_channel;
    Buffer.contents buffer;;

let split_sum str = 
    let split = Str.split (Str.regexp "\n") str in
    let split_num = List.map (fun x -> int_of_string x) split in
    List.fold_left (+) 0 split_num;;

let file_contents = read_file_into_string file;;
let split = (Str.split (Str.regexp "\n\n") file_contents);;
let split_sumS = List.map split_sum split;;

let myCompare x y = if x<y then 1 else -1;;
let sorted = List.sort myCompare split_sumS;;

printf "1: %d\n" (nth sorted 0);;
printf "2: %d\n" (nth sorted 1);;
printf "3: %d\n" (nth sorted 2);;

printf "sum top 3: %d\n" ((nth sorted 0) + (nth sorted 1) + (nth sorted 2));;
