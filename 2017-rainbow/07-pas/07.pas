program d07;

uses
	Sysutils;

const
	C_FNAME = '../input/07';


type
	node = ^TowerList;
	Tower = record
		ident: string;
		weight: integer;
		parent: ^Tower;
		children: node
	end;

	TowerList = record
		value: Tower;
		nxt: node;
	end;

var
	head: node;
	root: Tower;

function ParseLine (l: string): Tower;
var
	i: integer;
	s: integer;

	t: Tower;

begin
	for i := 1 to Length(l) do
	begin
		if l[i] = ' ' then
		begin
			s := i + 2;
			break;
		end;

		t.ident := t.ident + l[i];
	end;

	writeln (l[s..Length(l) - 1]);

	t.weight := StrToInt(l[s..Length(l) - 1]);
	ParseLine := t;
end;

Procedure CreateTower;
var
	tfIn: TextFile;
	inp: string;
	t: Tower;

begin
	Assign(tfIn, C_FNAME);
	reset(tfIn);

	new (head);
	head^.nxt := nil;

	while not eof(tfIn) do
	begin
		readln(tfIn, inp);
		writeln(inp);

		t := ParseLine (inp);
		writeln (t.ident, t.weight);
		break;
	end;

	Close(tfIn);
end;

begin
	CreateTower();

	while head <> nil do
		begin
			(* writeln (head^.value.ident); *)
			head := head^.nxt;
		end;
end.
