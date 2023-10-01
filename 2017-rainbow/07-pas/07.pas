program d07;

uses
	Sysutils;

const
	INP_FILE = '../input/07';

type
	node = ^TowerList;
	tPtr = ^Tower;

	Tower = record
		ident: string;
		weight: integer;
		parent: tPtr;
		children: node
	end;

	TowerList = record
		value: Tower;
		nxt: node;
	end;

var
	head: node;
	root: Tower;

function CreateChild (ident: string; parent: tPtr) : Tower;
var
	child: Tower;

begin
	child.ident := ident;
	child.parent := parent;

	CreateChild := child;
end;

function ParseLine (l: string) : Tower;
var
	i: integer;
	s: integer;
	id: string;

	t: Tower;
	ch: node;

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

	for i := s to Length(l) do
	begin
		if l[i] = ')' then
		begin
			t.weight := StrToInt(l[s..i-1]);
			s := i + 5;
			break;
		end;
	end;

	i := s;
	while i < Length(l) do
	begin
		id := '';
		while (i < Length(l)) and (not (l[i] = ',')) do
		begin
			id := id + l[i];
			i := i + 1;
		end;

		writeln (id[2]);
		i := i + 1;
	end;

	for i := s to Length(l) do
	begin
		writeln (l[i]);
	end;

	t.children := ch;
	ParseLine := t;
end;

Procedure CreateTower;
var
	tfIn: TextFile;
	inp: string;
	t: Tower;

begin
	Assign(tfIn, INP_FILE);
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
