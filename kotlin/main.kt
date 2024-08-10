package main
import java.io.File

fun main(args: Array<String>) {
   ReadLines(args[0]).filter{line -> args[1] in line}.forEach {
        println(it)
   }
}

fun ReadLines(fileName: String): List<String>
  = File(fileName).bufferedReader().readLines()
