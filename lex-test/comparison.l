%{
	enum {
		GT=1, GE, LT, LE, EQ, NE
	};
%}

%%

">" {return (GT);}
">=" {return (GE);}
"<" {return (LT);}
"<=" {return (LE);}
"=" {return (EQ);}
"<>" {return (NE);}

%%

int main() {
	yylex();
}