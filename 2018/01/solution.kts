import java.io.File

fun parse_input(): List<Int> {
    return File("input.txt")
        .readLines()
        .map({it.toInt()})
}

fun part1(input: Iterable<Int>): Int {
    return input.reduce({a, b -> a + b})
}

fun part2(input: Iterable<Int>): Int {
    var cumulative = 0
    val seen = mutableSetOf(0)
    while (true) {
        for (x in input) {
            cumulative += x
            if (seen.contains(cumulative)) {
                return cumulative
            }
            seen.add(cumulative)
        }
    }
}

val input = parse_input()
println(part1(input))
println(part2(input))
