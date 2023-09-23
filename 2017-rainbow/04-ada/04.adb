with Ada.Text_IO; use Ada.Text_IO;
with Ada.Containers.Vectors;

procedure Day_04 is
	F	   : File_Type;
	File_Name  : constant String := "../input/04";
	X	   : Integer := 0;

	function Part_One (Line : String) return Integer
	is
		Cur_Start	: Natural := 1;
		Cur_End		: Natural := 1;
		Next_Start 	: Natural := 1;
		Next_End 	: Natural := 1;
		Is_Valid	: Integer := 1;
	begin
		for I in Line'Range loop
			if Line (I) = ' ' then
				Cur_End := I - 1;
				Next_Start := I + 1;

				for J in I + 1 .. Line'Last loop
					if Line (J) = ' ' then
						Next_End := J - 1;

						if Line(Cur_Start .. Cur_End) = Line(Next_Start .. Next_End) then 
							Is_Valid := 0;
						end if;
						Next_Start := J + 1;
					end if;
				end loop;

				if Cur_Start /= Next_start and then Line(Cur_Start .. Cur_End) = Line(Next_Start .. Line'Last) then
					Is_Valid := 0;
				end if;

				Cur_Start := I + 1;
			end if;
		end loop;

		return Is_Valid;
	end Part_One;
begin
	Open (F, In_File, File_Name);

	while not End_Of_File (F) loop
		X := X + Part_One (Get_Line (F));
	end loop;

	Put_Line("Part One: " & Integer'Image(X));
	Close (F);
end Day_04;

