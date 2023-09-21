import std.stdio;
import std.file;
import std.string;
import std.conv;
import std.math;

class Point 
{
	int x, y;

	this(int x, int y)
	{
		this.x = x;
		this.y = y;
	}

	override size_t toHash() { return x * 1000 + y; }
	override bool opEquals(Object o) 
	{ 
		Point p = cast(Point) o;
		return p && x == p.x && y == p.y;
	}
}


Point ulam_spirale(int n) 
{
	int k = to!int(ceil((sqrt(to!float(n)) - 1) / 2));
	int t = 2*k+1;
	int m = t*t;
	t -= 1;

	if (n >= m-t) {
		return new Point(k - (m-n), -k);
	} else {
		m -= t;
	}

	if (n >= m - t) {
		return new Point(-k,-k+(m-n));
	} else {
		m -= t;
	}

	if (n >= m - t) {
		return new Point(-k+(m-n),k);
	} else {
		return new Point(k,k-(m-n-t));
	}
}

int part_one(int n) 
{
	Point point = ulam_spirale(n);
	return abs(point.x) + abs(point.y);
}

int part_two(int n) 
{
	int[Point] spiral = [new Point(0,0): 1];
	int[3] vec = [-1,0,1];
	int ptr = 2;

	for (;;)
	{
		int sum = 0;
		int* cur;

		Point p = ulam_spirale(ptr);

		foreach (x; vec) {
			foreach (y; vec) {
				cur = new Point(p.x + x, p.y + y) in spiral;
				if (cur !is null) { sum += *cur; } 
			}
		}

		if (sum > n) { return sum; }

		ptr += 1;
		spiral[p] = sum;
	}
}

void main() 
{
   int inp = to!int(readText("../input/03").strip());

   writefln("Part one: %d", part_one(inp));
   writefln("Part two: %d", part_two(inp));
}
