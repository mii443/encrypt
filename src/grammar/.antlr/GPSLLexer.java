// Generated from h:\Git\gpsl\src\grammar\GpslLexer.g4 by ANTLR 4.8
import org.antlr.v4.runtime.Lexer;
import org.antlr.v4.runtime.CharStream;
import org.antlr.v4.runtime.Token;
import org.antlr.v4.runtime.TokenStream;
import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.atn.*;
import org.antlr.v4.runtime.dfa.DFA;
import org.antlr.v4.runtime.misc.*;

@SuppressWarnings({"all", "warnings", "unchecked", "unused", "cast"})
public class GpslLexer extends Lexer {
	static { RuntimeMetaData.checkVersion("4.8", RuntimeMetaData.VERSION); }

	protected static final DFA[] _decisionToDFA;
	protected static final PredictionContextCache _sharedContextCache =
		new PredictionContextCache();
	public static final int
		WS=1, ADD=2, SUB=3, MUL=4, DIV=5, CONJ=6, AND=7, EQ=8, EQEQ=9, NE=10, 
		BE=11, LE=12, BT=13, LT=14, SEMICOLON=15, COLON=16, COMMA=17, DOT=18, 
		QUOTE=19, ADD_ASSIGNMENT=20, SUB_ASSIGNMENT=21, MUL_ASSIGNMENT=22, DIV_ASSIGNMENT=23, 
		LPAREN=24, RPAREN=25, LCURL=26, RCURL=27, ARROW=28, FN=29, FOR=30, WHILE=31, 
		IF=32, ELSE=33, LET=34, RETURN=35, NUM=36, TEXT=37, IDENT=38;
	public static String[] channelNames = {
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	};

	public static String[] modeNames = {
		"DEFAULT_MODE"
	};

	private static String[] makeRuleNames() {
		return new String[] {
			"WS", "ADD", "SUB", "MUL", "DIV", "CONJ", "AND", "EQ", "EQEQ", "NE", 
			"BE", "LE", "BT", "LT", "SEMICOLON", "COLON", "COMMA", "DOT", "QUOTE", 
			"ADD_ASSIGNMENT", "SUB_ASSIGNMENT", "MUL_ASSIGNMENT", "DIV_ASSIGNMENT", 
			"LPAREN", "RPAREN", "LCURL", "RCURL", "ARROW", "FN", "FOR", "WHILE", 
			"IF", "ELSE", "LET", "RETURN", "NUM", "TEXT", "IDENT"
		};
	}
	public static final String[] ruleNames = makeRuleNames();

	private static String[] makeLiteralNames() {
		return new String[] {
			null, null, "'+'", "'-'", "'*'", "'/'", "'&&'", "'&'", "'='", "'=='", 
			"'!='", "'>='", "'<='", "'>'", "'<'", "';'", "':'", "','", "'.'", "'\"'", 
			"'+='", "'-='", "'*='", "'/='", "'('", "')'", "'{'", "'}'", "'->'", "'fn'", 
			"'for'", "'while'", "'if'", "'else'", "'let'", "'return'"
		};
	}
	private static final String[] _LITERAL_NAMES = makeLiteralNames();
	private static String[] makeSymbolicNames() {
		return new String[] {
			null, "WS", "ADD", "SUB", "MUL", "DIV", "CONJ", "AND", "EQ", "EQEQ", 
			"NE", "BE", "LE", "BT", "LT", "SEMICOLON", "COLON", "COMMA", "DOT", "QUOTE", 
			"ADD_ASSIGNMENT", "SUB_ASSIGNMENT", "MUL_ASSIGNMENT", "DIV_ASSIGNMENT", 
			"LPAREN", "RPAREN", "LCURL", "RCURL", "ARROW", "FN", "FOR", "WHILE", 
			"IF", "ELSE", "LET", "RETURN", "NUM", "TEXT", "IDENT"
		};
	}
	private static final String[] _SYMBOLIC_NAMES = makeSymbolicNames();
	public static final Vocabulary VOCABULARY = new VocabularyImpl(_LITERAL_NAMES, _SYMBOLIC_NAMES);

	/**
	 * @deprecated Use {@link #VOCABULARY} instead.
	 */
	@Deprecated
	public static final String[] tokenNames;
	static {
		tokenNames = new String[_SYMBOLIC_NAMES.length];
		for (int i = 0; i < tokenNames.length; i++) {
			tokenNames[i] = VOCABULARY.getLiteralName(i);
			if (tokenNames[i] == null) {
				tokenNames[i] = VOCABULARY.getSymbolicName(i);
			}

			if (tokenNames[i] == null) {
				tokenNames[i] = "<INVALID>";
			}
		}
	}

	@Override
	@Deprecated
	public String[] getTokenNames() {
		return tokenNames;
	}

	@Override

	public Vocabulary getVocabulary() {
		return VOCABULARY;
	}


	public GpslLexer(CharStream input) {
		super(input);
		_interp = new LexerATNSimulator(this,_ATN,_decisionToDFA,_sharedContextCache);
	}

	@Override
	public String getGrammarFileName() { return "GpslLexer.g4"; }

	@Override
	public String[] getRuleNames() { return ruleNames; }

	@Override
	public String getSerializedATN() { return _serializedATN; }

	@Override
	public String[] getChannelNames() { return channelNames; }

	@Override
	public String[] getModeNames() { return modeNames; }

	@Override
	public ATN getATN() { return _ATN; }

	public static final String _serializedATN =
		"\3\u608b\ua72a\u8133\ub9ed\u417c\u3be7\u7786\u5964\2(\u00c8\b\1\4\2\t"+
		"\2\4\3\t\3\4\4\t\4\4\5\t\5\4\6\t\6\4\7\t\7\4\b\t\b\4\t\t\t\4\n\t\n\4\13"+
		"\t\13\4\f\t\f\4\r\t\r\4\16\t\16\4\17\t\17\4\20\t\20\4\21\t\21\4\22\t\22"+
		"\4\23\t\23\4\24\t\24\4\25\t\25\4\26\t\26\4\27\t\27\4\30\t\30\4\31\t\31"+
		"\4\32\t\32\4\33\t\33\4\34\t\34\4\35\t\35\4\36\t\36\4\37\t\37\4 \t \4!"+
		"\t!\4\"\t\"\4#\t#\4$\t$\4%\t%\4&\t&\4\'\t\'\3\2\3\2\3\2\3\2\3\3\3\3\3"+
		"\4\3\4\3\5\3\5\3\6\3\6\3\7\3\7\3\7\3\b\3\b\3\t\3\t\3\n\3\n\3\n\3\13\3"+
		"\13\3\13\3\f\3\f\3\f\3\r\3\r\3\r\3\16\3\16\3\17\3\17\3\20\3\20\3\21\3"+
		"\21\3\22\3\22\3\23\3\23\3\24\3\24\3\25\3\25\3\25\3\26\3\26\3\26\3\27\3"+
		"\27\3\27\3\30\3\30\3\30\3\31\3\31\3\32\3\32\3\33\3\33\3\34\3\34\3\35\3"+
		"\35\3\35\3\36\3\36\3\36\3\37\3\37\3\37\3\37\3 \3 \3 \3 \3 \3 \3!\3!\3"+
		"!\3\"\3\"\3\"\3\"\3\"\3#\3#\3#\3#\3$\3$\3$\3$\3$\3$\3$\3%\3%\7%\u00b6"+
		"\n%\f%\16%\u00b9\13%\3&\3&\7&\u00bd\n&\f&\16&\u00c0\13&\3&\3&\3\'\6\'"+
		"\u00c5\n\'\r\'\16\'\u00c6\2\2(\3\3\5\4\7\5\t\6\13\7\r\b\17\t\21\n\23\13"+
		"\25\f\27\r\31\16\33\17\35\20\37\21!\22#\23%\24\'\25)\26+\27-\30/\31\61"+
		"\32\63\33\65\34\67\359\36;\37= ?!A\"C#E$G%I&K\'M(\3\2\7\5\2\13\f\17\17"+
		"\"\"\3\2\63;\3\2\62;\7\2//\62;C\\aac|\5\2C\\aac|\2\u00ca\2\3\3\2\2\2\2"+
		"\5\3\2\2\2\2\7\3\2\2\2\2\t\3\2\2\2\2\13\3\2\2\2\2\r\3\2\2\2\2\17\3\2\2"+
		"\2\2\21\3\2\2\2\2\23\3\2\2\2\2\25\3\2\2\2\2\27\3\2\2\2\2\31\3\2\2\2\2"+
		"\33\3\2\2\2\2\35\3\2\2\2\2\37\3\2\2\2\2!\3\2\2\2\2#\3\2\2\2\2%\3\2\2\2"+
		"\2\'\3\2\2\2\2)\3\2\2\2\2+\3\2\2\2\2-\3\2\2\2\2/\3\2\2\2\2\61\3\2\2\2"+
		"\2\63\3\2\2\2\2\65\3\2\2\2\2\67\3\2\2\2\29\3\2\2\2\2;\3\2\2\2\2=\3\2\2"+
		"\2\2?\3\2\2\2\2A\3\2\2\2\2C\3\2\2\2\2E\3\2\2\2\2G\3\2\2\2\2I\3\2\2\2\2"+
		"K\3\2\2\2\2M\3\2\2\2\3O\3\2\2\2\5S\3\2\2\2\7U\3\2\2\2\tW\3\2\2\2\13Y\3"+
		"\2\2\2\r[\3\2\2\2\17^\3\2\2\2\21`\3\2\2\2\23b\3\2\2\2\25e\3\2\2\2\27h"+
		"\3\2\2\2\31k\3\2\2\2\33n\3\2\2\2\35p\3\2\2\2\37r\3\2\2\2!t\3\2\2\2#v\3"+
		"\2\2\2%x\3\2\2\2\'z\3\2\2\2)|\3\2\2\2+\177\3\2\2\2-\u0082\3\2\2\2/\u0085"+
		"\3\2\2\2\61\u0088\3\2\2\2\63\u008a\3\2\2\2\65\u008c\3\2\2\2\67\u008e\3"+
		"\2\2\29\u0090\3\2\2\2;\u0093\3\2\2\2=\u0096\3\2\2\2?\u009a\3\2\2\2A\u00a0"+
		"\3\2\2\2C\u00a3\3\2\2\2E\u00a8\3\2\2\2G\u00ac\3\2\2\2I\u00b3\3\2\2\2K"+
		"\u00ba\3\2\2\2M\u00c4\3\2\2\2OP\t\2\2\2PQ\3\2\2\2QR\b\2\2\2R\4\3\2\2\2"+
		"ST\7-\2\2T\6\3\2\2\2UV\7/\2\2V\b\3\2\2\2WX\7,\2\2X\n\3\2\2\2YZ\7\61\2"+
		"\2Z\f\3\2\2\2[\\\7(\2\2\\]\7(\2\2]\16\3\2\2\2^_\7(\2\2_\20\3\2\2\2`a\7"+
		"?\2\2a\22\3\2\2\2bc\7?\2\2cd\7?\2\2d\24\3\2\2\2ef\7#\2\2fg\7?\2\2g\26"+
		"\3\2\2\2hi\7@\2\2ij\7?\2\2j\30\3\2\2\2kl\7>\2\2lm\7?\2\2m\32\3\2\2\2n"+
		"o\7@\2\2o\34\3\2\2\2pq\7>\2\2q\36\3\2\2\2rs\7=\2\2s \3\2\2\2tu\7<\2\2"+
		"u\"\3\2\2\2vw\7.\2\2w$\3\2\2\2xy\7\60\2\2y&\3\2\2\2z{\7$\2\2{(\3\2\2\2"+
		"|}\7-\2\2}~\7?\2\2~*\3\2\2\2\177\u0080\7/\2\2\u0080\u0081\7?\2\2\u0081"+
		",\3\2\2\2\u0082\u0083\7,\2\2\u0083\u0084\7?\2\2\u0084.\3\2\2\2\u0085\u0086"+
		"\7\61\2\2\u0086\u0087\7?\2\2\u0087\60\3\2\2\2\u0088\u0089\7*\2\2\u0089"+
		"\62\3\2\2\2\u008a\u008b\7+\2\2\u008b\64\3\2\2\2\u008c\u008d\7}\2\2\u008d"+
		"\66\3\2\2\2\u008e\u008f\7\177\2\2\u008f8\3\2\2\2\u0090\u0091\7/\2\2\u0091"+
		"\u0092\7@\2\2\u0092:\3\2\2\2\u0093\u0094\7h\2\2\u0094\u0095\7p\2\2\u0095"+
		"<\3\2\2\2\u0096\u0097\7h\2\2\u0097\u0098\7q\2\2\u0098\u0099\7t\2\2\u0099"+
		">\3\2\2\2\u009a\u009b\7y\2\2\u009b\u009c\7j\2\2\u009c\u009d\7k\2\2\u009d"+
		"\u009e\7n\2\2\u009e\u009f\7g\2\2\u009f@\3\2\2\2\u00a0\u00a1\7k\2\2\u00a1"+
		"\u00a2\7h\2\2\u00a2B\3\2\2\2\u00a3\u00a4\7g\2\2\u00a4\u00a5\7n\2\2\u00a5"+
		"\u00a6\7u\2\2\u00a6\u00a7\7g\2\2\u00a7D\3\2\2\2\u00a8\u00a9\7n\2\2\u00a9"+
		"\u00aa\7g\2\2\u00aa\u00ab\7v\2\2\u00abF\3\2\2\2\u00ac\u00ad\7t\2\2\u00ad"+
		"\u00ae\7g\2\2\u00ae\u00af\7v\2\2\u00af\u00b0\7w\2\2\u00b0\u00b1\7t\2\2"+
		"\u00b1\u00b2\7p\2\2\u00b2H\3\2\2\2\u00b3\u00b7\t\3\2\2\u00b4\u00b6\t\4"+
		"\2\2\u00b5\u00b4\3\2\2\2\u00b6\u00b9\3\2\2\2\u00b7\u00b5\3\2\2\2\u00b7"+
		"\u00b8\3\2\2\2\u00b8J\3\2\2\2\u00b9\u00b7\3\2\2\2\u00ba\u00be\5\'\24\2"+
		"\u00bb\u00bd\t\5\2\2\u00bc\u00bb\3\2\2\2\u00bd\u00c0\3\2\2\2\u00be\u00bc"+
		"\3\2\2\2\u00be\u00bf\3\2\2\2\u00bf\u00c1\3\2\2\2\u00c0\u00be\3\2\2\2\u00c1"+
		"\u00c2\5\'\24\2\u00c2L\3\2\2\2\u00c3\u00c5\t\6\2\2\u00c4\u00c3\3\2\2\2"+
		"\u00c5\u00c6\3\2\2\2\u00c6\u00c4\3\2\2\2\u00c6\u00c7\3\2\2\2\u00c7N\3"+
		"\2\2\2\6\2\u00b7\u00be\u00c6\3\b\2\2";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}