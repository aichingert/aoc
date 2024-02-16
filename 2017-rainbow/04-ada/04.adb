with Ada.Text_IO; use Ada.Text_IO;
with Ada.Containers; use Ada.Containers;
with Ada.Containers.Vectors;

procedure Day_04 is
	type Integer_Array is
		array (1 .. 26) of Integer;

	package Array_Vectors is new Ada.Containers.Vectors 
		(Index_Type => Natural,
		 Element_Type => Integer_Array);
	use Array_Vectors;

	F          : File_Type;
	File_Name  : constant String := "../input/04";

	Answer_One : Integer         := 0;
	Answer_Two : Integer         := 0;

	function Part_One (Line : String) return Integer 
    is
		fs,  fe, ss, se : Natural := 1;
	begin
		for I in Line'Range loop
			if Line (I) = ' ' then
				fe := I - 1;
				ss := I + 1;

				for J in I + 1 .. Line'Last loop
					if Line (J) = ' ' then
						se := J - 1;
						if Line (fs .. fe) = Line (ss .. se) then return 0; end if;
						ss := J + 1;
					end if;
				end loop;

				if fs /= ss and then Line (fs .. fe) = Line(ss .. Line'Last) then return 0; end if;
				fs:= I + 1;
			end if;
		end loop;

		return 1;
	end Part_One;

	function Part_Two (Line : String) return Integer
	is
		V 	: Vector;
		Index	: Natural := 1;
		Offset	: Natural := Character'Pos ('a') - 1;
	begin
		loop
			declare 
				Counter	: Integer_Array := (others => 0);
				Idx 	: Natural       := 1;
			begin
				for I in Index .. Line'Last loop
					if Line (I) = ' ' then
						V.Append (Counter);
						Index := I + 1;
						exit;
					else
						Idx := Character'Pos (Line (I)) - Offset;
						Counter (Idx) := Counter (Idx) + 1;

						if I = Line'Last then
							Index := Line'Last + 1;
							V.Append (Counter);
						end if;
	 				end if;
				end loop;
			end;

			exit when Index > Line'Last;
		end loop;

		for I in V.First_Index .. V.Last_Index - 1 loop
			for J in I + 1 .. V.Last_Index loop
				declare 
					This	: constant Integer_Array := V (I);
					Other 	: constant Integer_Array := V (J);
					Index 	: Natural                := 1;
					Test	: Integer                := 1;
				begin
					loop 
						if This (Index) /= Other (Index) then
							Test := Test + 1;
							exit;
						end if;

						Index := Index + 1;
						exit when Index > This'Last;
					end loop;

					if Index > This'Last then return 0; end if;
				end;
			end loop;
		end loop;

		return 1;
	end Part_Two;

begin
	Open (F, In_File, File_Name);

	while not End_Of_File (F) loop
		declare
			Line : String := Get_Line (F);
		begin
			Answer_One := Answer_One + Part_One (Line);
			Answer_Two := Answer_Two + Part_Two (Line);
		end;
	end loop;

	Put_Line ("Part one: " & Integer'Image (Answer_One));
	Put_Line ("Part two: " & Integer'Image (Answer_Two));
	Close (F);
end Day_04;
