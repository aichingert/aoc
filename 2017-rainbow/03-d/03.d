import std.stdio;
import std.file;
import std.string;
import std.conv;

int[2] ulam_spirale(int n) {
   return [0, 0];
}

int part_one(int n) {
   return -1;
}

int part_two(int n) {
   return -1;
}

void main() {
   int inp = to!int(readText("../input/03").strip());

   writefln("Part one: %d", part_one(inp));
   writefln("Part two: %d", part_two(inp));
}
