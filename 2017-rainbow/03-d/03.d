import std.stdio;
import std.file;
import std.string;
import std.conv;
import std.math;

int[2] ulam_spirale(int n) {
	int k = to!int(ceil((sqrt(to!float(n)) - 1) / 2));
	int t = 2*k+1;
	int m = t*t;
	t -= 1;

	if (n >= m-t) {
		return [k - (m-n), -k];
	} else {
		m -= t;
	}

	if (n >= m - t) {
		return [-k,-k+(m-n)];
	} else {
		m -= t;
	}

	if (n >= m - t) {
		return [-k+(m-n),k];
	} else {
		return [k,k-(m-n-t)];
	}
}

int part_one(int n) {
	int[2] cords = ulam_spirale(n);
	return abs(cords[0]) + abs(cords[1]);
}

int part_two(int n) {
   	return -1;
}

void main() {
   int inp = to!int(readText("../input/03").strip());

   writefln("Part one: %d", part_one(inp));
   writefln("Part two: %d", part_two(inp));
}
