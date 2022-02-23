import Debug "mo:base/Debug";
import Nat "mo:base/Nat";
import Int "mo:base/Int";
import Array "mo:base/Array";

actor {
    func quicksort(arr:[var Int]):() {
        sort(arr, 0, arr.size()-1);
    };
    func sort(arr:[var Int], start:Nat, end:Nat):() {
        var i = start;
        var j = end;
        var temp = arr[start];
        if (i < j) {
            while(i < j) {
                while(i < j and arr[j] >= temp){
                    j := j - 1;
                };
                if(i < j) {
                    arr[i] := arr[j];
                    i := i + 1;
                };
                while(i < j and arr[i] < temp) {
                    i := i + 1;
                };
                if(i < j) {
                    arr[j] := arr[i];
                    j := j - 1;
                };
            };
            arr[i] := temp;
            if(start < i){
                sort(arr, start, j - 1);
            };
            if(i < end){
                sort(arr, j + 1, end);
            };
        };
    };
    public func qsort(arr: [Int]):async [Int] {
        // 1.transform immutable array into mutable array
        let res:[var Int] = Array.thaw(arr);
        // 2.quicksort
        quicksort(res);
        // 3.transform mutable array into immutable array
        Array.freeze<Int>(res);
    };
}

//===============================================================
// func quicksort(arr:[var Int]):() {
//         sort(arr, 0, arr.size()-1);
//     };

//     func sort(arr:[var Int], start:Nat, end:Nat):() {
//         var i = start;
//         var j = end;
//         var temp = arr[start];
//         if (i < j) {
//             while(i < j) {
//                 while(i < j and arr[j] >= temp){
//                     j := j - 1;
//                 };
//                 if(i < j) {
//                     arr[i] := arr[j];
//                     i := i + 1;
//                 };
//                 while(i < j and arr[i] < temp) {
//                     i := i + 1;
//                 };
//                 if(i < j) {
//                     arr[j] := arr[i];
//                     j := j - 1;
//                 };
//             };
//             arr[i] := temp;
//             if(start < i){
//                 sort(arr, start, j - 1);
//             };
//             if(i < end){
//                 sort(arr, j + 1, end);
//             };
//         };
//     };


// let a:[var Int] = [var 3,1,2];
// quicksort(a);
// // Debug.print(Int.toText(1));

// for (i in a.vals()) {
//     Debug.print(Int.toText(i));
// };
