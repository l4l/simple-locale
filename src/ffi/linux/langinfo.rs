/* automatically generated by rust-bindgen */

pub const _LANGINFO_H : u32 = 1 ; pub const _NL_TYPES_H : u32 = 1 ; pub const _FEATURES_H : u32 = 1 ; pub const _DEFAULT_SOURCE : u32 = 1 ; pub const __USE_ISOC11 : u32 = 1 ; pub const __USE_ISOC99 : u32 = 1 ; pub const __USE_ISOC95 : u32 = 1 ; pub const __USE_POSIX_IMPLICITLY : u32 = 1 ; pub const _POSIX_SOURCE : u32 = 1 ; pub const _POSIX_C_SOURCE : u32 = 200809 ; pub const __USE_POSIX : u32 = 1 ; pub const __USE_POSIX2 : u32 = 1 ; pub const __USE_POSIX199309 : u32 = 1 ; pub const __USE_POSIX199506 : u32 = 1 ; pub const __USE_XOPEN2K : u32 = 1 ; pub const __USE_XOPEN2K8 : u32 = 1 ; pub const _ATFILE_SOURCE : u32 = 1 ; pub const __USE_MISC : u32 = 1 ; pub const __USE_ATFILE : u32 = 1 ; pub const __USE_FORTIFY_LEVEL : u32 = 0 ; pub const __GLIBC_USE_DEPRECATED_GETS : u32 = 0 ; pub const __GLIBC_USE_DEPRECATED_SCANF : u32 = 0 ; pub const _STDC_PREDEF_H : u32 = 1 ; pub const __STDC_IEC_559__ : u32 = 1 ; pub const __STDC_IEC_559_COMPLEX__ : u32 = 1 ; pub const __STDC_ISO_10646__ : u32 = 201706 ; pub const __GNU_LIBRARY__ : u32 = 6 ; pub const __GLIBC__ : u32 = 2 ; pub const __GLIBC_MINOR__ : u32 = 29 ; pub const _SYS_CDEFS_H : u32 = 1 ; pub const __glibc_c99_flexarr_available : u32 = 1 ; pub const __WORDSIZE : u32 = 64 ; pub const __WORDSIZE_TIME64_COMPAT32 : u32 = 1 ; pub const __SYSCALL_WORDSIZE : u32 = 64 ; pub const __HAVE_GENERIC_SELECTION : u32 = 1 ; pub const NL_SETD : u32 = 1 ; pub const NL_CAT_LOCALE : u32 = 1 ; pub const _BITS_LOCALE_H : u32 = 1 ; pub const __LC_CTYPE : u32 = 0 ; pub const __LC_NUMERIC : u32 = 1 ; pub const __LC_TIME : u32 = 2 ; pub const __LC_COLLATE : u32 = 3 ; pub const __LC_MONETARY : u32 = 4 ; pub const __LC_MESSAGES : u32 = 5 ; pub const __LC_ALL : u32 = 6 ; pub const __LC_PAPER : u32 = 7 ; pub const __LC_NAME : u32 = 8 ; pub const __LC_ADDRESS : u32 = 9 ; pub const __LC_TELEPHONE : u32 = 10 ; pub const __LC_MEASUREMENT : u32 = 11 ; pub const __LC_IDENTIFICATION : u32 = 12 ; pub const _BITS_TYPES_LOCALE_T_H : u32 = 1 ; pub const _BITS_TYPES___LOCALE_T_H : u32 = 1 ; pub type nl_catd = * mut :: std :: os :: raw :: c_void ; pub type nl_item = :: std :: os :: raw :: c_int ; extern "C" { pub fn catopen ( __cat_name : * const :: std :: os :: raw :: c_char , __flag : :: std :: os :: raw :: c_int ) -> nl_catd ; } extern "C" { pub fn catgets ( __catalog : nl_catd , __set : :: std :: os :: raw :: c_int , __number : :: std :: os :: raw :: c_int , __string : * const :: std :: os :: raw :: c_char ) -> * mut :: std :: os :: raw :: c_char ; } extern "C" { pub fn catclose ( __catalog : nl_catd ) -> :: std :: os :: raw :: c_int ; } pub const ABDAY_1 : _bindgen_ty_1 = 131072 ; pub const ABDAY_2 : _bindgen_ty_1 = 131073 ; pub const ABDAY_3 : _bindgen_ty_1 = 131074 ; pub const ABDAY_4 : _bindgen_ty_1 = 131075 ; pub const ABDAY_5 : _bindgen_ty_1 = 131076 ; pub const ABDAY_6 : _bindgen_ty_1 = 131077 ; pub const ABDAY_7 : _bindgen_ty_1 = 131078 ; pub const DAY_1 : _bindgen_ty_1 = 131079 ; pub const DAY_2 : _bindgen_ty_1 = 131080 ; pub const DAY_3 : _bindgen_ty_1 = 131081 ; pub const DAY_4 : _bindgen_ty_1 = 131082 ; pub const DAY_5 : _bindgen_ty_1 = 131083 ; pub const DAY_6 : _bindgen_ty_1 = 131084 ; pub const DAY_7 : _bindgen_ty_1 = 131085 ; pub const ABMON_1 : _bindgen_ty_1 = 131086 ; pub const ABMON_2 : _bindgen_ty_1 = 131087 ; pub const ABMON_3 : _bindgen_ty_1 = 131088 ; pub const ABMON_4 : _bindgen_ty_1 = 131089 ; pub const ABMON_5 : _bindgen_ty_1 = 131090 ; pub const ABMON_6 : _bindgen_ty_1 = 131091 ; pub const ABMON_7 : _bindgen_ty_1 = 131092 ; pub const ABMON_8 : _bindgen_ty_1 = 131093 ; pub const ABMON_9 : _bindgen_ty_1 = 131094 ; pub const ABMON_10 : _bindgen_ty_1 = 131095 ; pub const ABMON_11 : _bindgen_ty_1 = 131096 ; pub const ABMON_12 : _bindgen_ty_1 = 131097 ; pub const MON_1 : _bindgen_ty_1 = 131098 ; pub const MON_2 : _bindgen_ty_1 = 131099 ; pub const MON_3 : _bindgen_ty_1 = 131100 ; pub const MON_4 : _bindgen_ty_1 = 131101 ; pub const MON_5 : _bindgen_ty_1 = 131102 ; pub const MON_6 : _bindgen_ty_1 = 131103 ; pub const MON_7 : _bindgen_ty_1 = 131104 ; pub const MON_8 : _bindgen_ty_1 = 131105 ; pub const MON_9 : _bindgen_ty_1 = 131106 ; pub const MON_10 : _bindgen_ty_1 = 131107 ; pub const MON_11 : _bindgen_ty_1 = 131108 ; pub const MON_12 : _bindgen_ty_1 = 131109 ; pub const AM_STR : _bindgen_ty_1 = 131110 ; pub const PM_STR : _bindgen_ty_1 = 131111 ; pub const D_T_FMT : _bindgen_ty_1 = 131112 ; pub const D_FMT : _bindgen_ty_1 = 131113 ; pub const T_FMT : _bindgen_ty_1 = 131114 ; pub const T_FMT_AMPM : _bindgen_ty_1 = 131115 ; pub const ERA : _bindgen_ty_1 = 131116 ; pub const __ERA_YEAR : _bindgen_ty_1 = 131117 ; pub const ERA_D_FMT : _bindgen_ty_1 = 131118 ; pub const ALT_DIGITS : _bindgen_ty_1 = 131119 ; pub const ERA_D_T_FMT : _bindgen_ty_1 = 131120 ; pub const ERA_T_FMT : _bindgen_ty_1 = 131121 ; pub const _NL_TIME_ERA_NUM_ENTRIES : _bindgen_ty_1 = 131122 ; pub const _NL_TIME_ERA_ENTRIES : _bindgen_ty_1 = 131123 ; pub const _NL_WABDAY_1 : _bindgen_ty_1 = 131124 ; pub const _NL_WABDAY_2 : _bindgen_ty_1 = 131125 ; pub const _NL_WABDAY_3 : _bindgen_ty_1 = 131126 ; pub const _NL_WABDAY_4 : _bindgen_ty_1 = 131127 ; pub const _NL_WABDAY_5 : _bindgen_ty_1 = 131128 ; pub const _NL_WABDAY_6 : _bindgen_ty_1 = 131129 ; pub const _NL_WABDAY_7 : _bindgen_ty_1 = 131130 ; pub const _NL_WDAY_1 : _bindgen_ty_1 = 131131 ; pub const _NL_WDAY_2 : _bindgen_ty_1 = 131132 ; pub const _NL_WDAY_3 : _bindgen_ty_1 = 131133 ; pub const _NL_WDAY_4 : _bindgen_ty_1 = 131134 ; pub const _NL_WDAY_5 : _bindgen_ty_1 = 131135 ; pub const _NL_WDAY_6 : _bindgen_ty_1 = 131136 ; pub const _NL_WDAY_7 : _bindgen_ty_1 = 131137 ; pub const _NL_WABMON_1 : _bindgen_ty_1 = 131138 ; pub const _NL_WABMON_2 : _bindgen_ty_1 = 131139 ; pub const _NL_WABMON_3 : _bindgen_ty_1 = 131140 ; pub const _NL_WABMON_4 : _bindgen_ty_1 = 131141 ; pub const _NL_WABMON_5 : _bindgen_ty_1 = 131142 ; pub const _NL_WABMON_6 : _bindgen_ty_1 = 131143 ; pub const _NL_WABMON_7 : _bindgen_ty_1 = 131144 ; pub const _NL_WABMON_8 : _bindgen_ty_1 = 131145 ; pub const _NL_WABMON_9 : _bindgen_ty_1 = 131146 ; pub const _NL_WABMON_10 : _bindgen_ty_1 = 131147 ; pub const _NL_WABMON_11 : _bindgen_ty_1 = 131148 ; pub const _NL_WABMON_12 : _bindgen_ty_1 = 131149 ; pub const _NL_WMON_1 : _bindgen_ty_1 = 131150 ; pub const _NL_WMON_2 : _bindgen_ty_1 = 131151 ; pub const _NL_WMON_3 : _bindgen_ty_1 = 131152 ; pub const _NL_WMON_4 : _bindgen_ty_1 = 131153 ; pub const _NL_WMON_5 : _bindgen_ty_1 = 131154 ; pub const _NL_WMON_6 : _bindgen_ty_1 = 131155 ; pub const _NL_WMON_7 : _bindgen_ty_1 = 131156 ; pub const _NL_WMON_8 : _bindgen_ty_1 = 131157 ; pub const _NL_WMON_9 : _bindgen_ty_1 = 131158 ; pub const _NL_WMON_10 : _bindgen_ty_1 = 131159 ; pub const _NL_WMON_11 : _bindgen_ty_1 = 131160 ; pub const _NL_WMON_12 : _bindgen_ty_1 = 131161 ; pub const _NL_WAM_STR : _bindgen_ty_1 = 131162 ; pub const _NL_WPM_STR : _bindgen_ty_1 = 131163 ; pub const _NL_WD_T_FMT : _bindgen_ty_1 = 131164 ; pub const _NL_WD_FMT : _bindgen_ty_1 = 131165 ; pub const _NL_WT_FMT : _bindgen_ty_1 = 131166 ; pub const _NL_WT_FMT_AMPM : _bindgen_ty_1 = 131167 ; pub const _NL_WERA_YEAR : _bindgen_ty_1 = 131168 ; pub const _NL_WERA_D_FMT : _bindgen_ty_1 = 131169 ; pub const _NL_WALT_DIGITS : _bindgen_ty_1 = 131170 ; pub const _NL_WERA_D_T_FMT : _bindgen_ty_1 = 131171 ; pub const _NL_WERA_T_FMT : _bindgen_ty_1 = 131172 ; pub const _NL_TIME_WEEK_NDAYS : _bindgen_ty_1 = 131173 ; pub const _NL_TIME_WEEK_1STDAY : _bindgen_ty_1 = 131174 ; pub const _NL_TIME_WEEK_1STWEEK : _bindgen_ty_1 = 131175 ; pub const _NL_TIME_FIRST_WEEKDAY : _bindgen_ty_1 = 131176 ; pub const _NL_TIME_FIRST_WORKDAY : _bindgen_ty_1 = 131177 ; pub const _NL_TIME_CAL_DIRECTION : _bindgen_ty_1 = 131178 ; pub const _NL_TIME_TIMEZONE : _bindgen_ty_1 = 131179 ; pub const _DATE_FMT : _bindgen_ty_1 = 131180 ; pub const _NL_W_DATE_FMT : _bindgen_ty_1 = 131181 ; pub const _NL_TIME_CODESET : _bindgen_ty_1 = 131182 ; pub const __ALTMON_1 : _bindgen_ty_1 = 131183 ; pub const __ALTMON_2 : _bindgen_ty_1 = 131184 ; pub const __ALTMON_3 : _bindgen_ty_1 = 131185 ; pub const __ALTMON_4 : _bindgen_ty_1 = 131186 ; pub const __ALTMON_5 : _bindgen_ty_1 = 131187 ; pub const __ALTMON_6 : _bindgen_ty_1 = 131188 ; pub const __ALTMON_7 : _bindgen_ty_1 = 131189 ; pub const __ALTMON_8 : _bindgen_ty_1 = 131190 ; pub const __ALTMON_9 : _bindgen_ty_1 = 131191 ; pub const __ALTMON_10 : _bindgen_ty_1 = 131192 ; pub const __ALTMON_11 : _bindgen_ty_1 = 131193 ; pub const __ALTMON_12 : _bindgen_ty_1 = 131194 ; pub const _NL_WALTMON_1 : _bindgen_ty_1 = 131195 ; pub const _NL_WALTMON_2 : _bindgen_ty_1 = 131196 ; pub const _NL_WALTMON_3 : _bindgen_ty_1 = 131197 ; pub const _NL_WALTMON_4 : _bindgen_ty_1 = 131198 ; pub const _NL_WALTMON_5 : _bindgen_ty_1 = 131199 ; pub const _NL_WALTMON_6 : _bindgen_ty_1 = 131200 ; pub const _NL_WALTMON_7 : _bindgen_ty_1 = 131201 ; pub const _NL_WALTMON_8 : _bindgen_ty_1 = 131202 ; pub const _NL_WALTMON_9 : _bindgen_ty_1 = 131203 ; pub const _NL_WALTMON_10 : _bindgen_ty_1 = 131204 ; pub const _NL_WALTMON_11 : _bindgen_ty_1 = 131205 ; pub const _NL_WALTMON_12 : _bindgen_ty_1 = 131206 ; pub const _NL_ABALTMON_1 : _bindgen_ty_1 = 131207 ; pub const _NL_ABALTMON_2 : _bindgen_ty_1 = 131208 ; pub const _NL_ABALTMON_3 : _bindgen_ty_1 = 131209 ; pub const _NL_ABALTMON_4 : _bindgen_ty_1 = 131210 ; pub const _NL_ABALTMON_5 : _bindgen_ty_1 = 131211 ; pub const _NL_ABALTMON_6 : _bindgen_ty_1 = 131212 ; pub const _NL_ABALTMON_7 : _bindgen_ty_1 = 131213 ; pub const _NL_ABALTMON_8 : _bindgen_ty_1 = 131214 ; pub const _NL_ABALTMON_9 : _bindgen_ty_1 = 131215 ; pub const _NL_ABALTMON_10 : _bindgen_ty_1 = 131216 ; pub const _NL_ABALTMON_11 : _bindgen_ty_1 = 131217 ; pub const _NL_ABALTMON_12 : _bindgen_ty_1 = 131218 ; pub const _NL_WABALTMON_1 : _bindgen_ty_1 = 131219 ; pub const _NL_WABALTMON_2 : _bindgen_ty_1 = 131220 ; pub const _NL_WABALTMON_3 : _bindgen_ty_1 = 131221 ; pub const _NL_WABALTMON_4 : _bindgen_ty_1 = 131222 ; pub const _NL_WABALTMON_5 : _bindgen_ty_1 = 131223 ; pub const _NL_WABALTMON_6 : _bindgen_ty_1 = 131224 ; pub const _NL_WABALTMON_7 : _bindgen_ty_1 = 131225 ; pub const _NL_WABALTMON_8 : _bindgen_ty_1 = 131226 ; pub const _NL_WABALTMON_9 : _bindgen_ty_1 = 131227 ; pub const _NL_WABALTMON_10 : _bindgen_ty_1 = 131228 ; pub const _NL_WABALTMON_11 : _bindgen_ty_1 = 131229 ; pub const _NL_WABALTMON_12 : _bindgen_ty_1 = 131230 ; pub const _NL_NUM_LC_TIME : _bindgen_ty_1 = 131231 ; pub const _NL_COLLATE_NRULES : _bindgen_ty_1 = 196608 ; pub const _NL_COLLATE_RULESETS : _bindgen_ty_1 = 196609 ; pub const _NL_COLLATE_TABLEMB : _bindgen_ty_1 = 196610 ; pub const _NL_COLLATE_WEIGHTMB : _bindgen_ty_1 = 196611 ; pub const _NL_COLLATE_EXTRAMB : _bindgen_ty_1 = 196612 ; pub const _NL_COLLATE_INDIRECTMB : _bindgen_ty_1 = 196613 ; pub const _NL_COLLATE_GAP1 : _bindgen_ty_1 = 196614 ; pub const _NL_COLLATE_GAP2 : _bindgen_ty_1 = 196615 ; pub const _NL_COLLATE_GAP3 : _bindgen_ty_1 = 196616 ; pub const _NL_COLLATE_TABLEWC : _bindgen_ty_1 = 196617 ; pub const _NL_COLLATE_WEIGHTWC : _bindgen_ty_1 = 196618 ; pub const _NL_COLLATE_EXTRAWC : _bindgen_ty_1 = 196619 ; pub const _NL_COLLATE_INDIRECTWC : _bindgen_ty_1 = 196620 ; pub const _NL_COLLATE_SYMB_HASH_SIZEMB : _bindgen_ty_1 = 196621 ; pub const _NL_COLLATE_SYMB_TABLEMB : _bindgen_ty_1 = 196622 ; pub const _NL_COLLATE_SYMB_EXTRAMB : _bindgen_ty_1 = 196623 ; pub const _NL_COLLATE_COLLSEQMB : _bindgen_ty_1 = 196624 ; pub const _NL_COLLATE_COLLSEQWC : _bindgen_ty_1 = 196625 ; pub const _NL_COLLATE_CODESET : _bindgen_ty_1 = 196626 ; pub const _NL_NUM_LC_COLLATE : _bindgen_ty_1 = 196627 ; pub const _NL_CTYPE_CLASS : _bindgen_ty_1 = 0 ; pub const _NL_CTYPE_TOUPPER : _bindgen_ty_1 = 1 ; pub const _NL_CTYPE_GAP1 : _bindgen_ty_1 = 2 ; pub const _NL_CTYPE_TOLOWER : _bindgen_ty_1 = 3 ; pub const _NL_CTYPE_GAP2 : _bindgen_ty_1 = 4 ; pub const _NL_CTYPE_CLASS32 : _bindgen_ty_1 = 5 ; pub const _NL_CTYPE_GAP3 : _bindgen_ty_1 = 6 ; pub const _NL_CTYPE_GAP4 : _bindgen_ty_1 = 7 ; pub const _NL_CTYPE_GAP5 : _bindgen_ty_1 = 8 ; pub const _NL_CTYPE_GAP6 : _bindgen_ty_1 = 9 ; pub const _NL_CTYPE_CLASS_NAMES : _bindgen_ty_1 = 10 ; pub const _NL_CTYPE_MAP_NAMES : _bindgen_ty_1 = 11 ; pub const _NL_CTYPE_WIDTH : _bindgen_ty_1 = 12 ; pub const _NL_CTYPE_MB_CUR_MAX : _bindgen_ty_1 = 13 ; pub const _NL_CTYPE_CODESET_NAME : _bindgen_ty_1 = 14 ; pub const CODESET : _bindgen_ty_1 = 14 ; pub const _NL_CTYPE_TOUPPER32 : _bindgen_ty_1 = 15 ; pub const _NL_CTYPE_TOLOWER32 : _bindgen_ty_1 = 16 ; pub const _NL_CTYPE_CLASS_OFFSET : _bindgen_ty_1 = 17 ; pub const _NL_CTYPE_MAP_OFFSET : _bindgen_ty_1 = 18 ; pub const _NL_CTYPE_INDIGITS_MB_LEN : _bindgen_ty_1 = 19 ; pub const _NL_CTYPE_INDIGITS0_MB : _bindgen_ty_1 = 20 ; pub const _NL_CTYPE_INDIGITS1_MB : _bindgen_ty_1 = 21 ; pub const _NL_CTYPE_INDIGITS2_MB : _bindgen_ty_1 = 22 ; pub const _NL_CTYPE_INDIGITS3_MB : _bindgen_ty_1 = 23 ; pub const _NL_CTYPE_INDIGITS4_MB : _bindgen_ty_1 = 24 ; pub const _NL_CTYPE_INDIGITS5_MB : _bindgen_ty_1 = 25 ; pub const _NL_CTYPE_INDIGITS6_MB : _bindgen_ty_1 = 26 ; pub const _NL_CTYPE_INDIGITS7_MB : _bindgen_ty_1 = 27 ; pub const _NL_CTYPE_INDIGITS8_MB : _bindgen_ty_1 = 28 ; pub const _NL_CTYPE_INDIGITS9_MB : _bindgen_ty_1 = 29 ; pub const _NL_CTYPE_INDIGITS_WC_LEN : _bindgen_ty_1 = 30 ; pub const _NL_CTYPE_INDIGITS0_WC : _bindgen_ty_1 = 31 ; pub const _NL_CTYPE_INDIGITS1_WC : _bindgen_ty_1 = 32 ; pub const _NL_CTYPE_INDIGITS2_WC : _bindgen_ty_1 = 33 ; pub const _NL_CTYPE_INDIGITS3_WC : _bindgen_ty_1 = 34 ; pub const _NL_CTYPE_INDIGITS4_WC : _bindgen_ty_1 = 35 ; pub const _NL_CTYPE_INDIGITS5_WC : _bindgen_ty_1 = 36 ; pub const _NL_CTYPE_INDIGITS6_WC : _bindgen_ty_1 = 37 ; pub const _NL_CTYPE_INDIGITS7_WC : _bindgen_ty_1 = 38 ; pub const _NL_CTYPE_INDIGITS8_WC : _bindgen_ty_1 = 39 ; pub const _NL_CTYPE_INDIGITS9_WC : _bindgen_ty_1 = 40 ; pub const _NL_CTYPE_OUTDIGIT0_MB : _bindgen_ty_1 = 41 ; pub const _NL_CTYPE_OUTDIGIT1_MB : _bindgen_ty_1 = 42 ; pub const _NL_CTYPE_OUTDIGIT2_MB : _bindgen_ty_1 = 43 ; pub const _NL_CTYPE_OUTDIGIT3_MB : _bindgen_ty_1 = 44 ; pub const _NL_CTYPE_OUTDIGIT4_MB : _bindgen_ty_1 = 45 ; pub const _NL_CTYPE_OUTDIGIT5_MB : _bindgen_ty_1 = 46 ; pub const _NL_CTYPE_OUTDIGIT6_MB : _bindgen_ty_1 = 47 ; pub const _NL_CTYPE_OUTDIGIT7_MB : _bindgen_ty_1 = 48 ; pub const _NL_CTYPE_OUTDIGIT8_MB : _bindgen_ty_1 = 49 ; pub const _NL_CTYPE_OUTDIGIT9_MB : _bindgen_ty_1 = 50 ; pub const _NL_CTYPE_OUTDIGIT0_WC : _bindgen_ty_1 = 51 ; pub const _NL_CTYPE_OUTDIGIT1_WC : _bindgen_ty_1 = 52 ; pub const _NL_CTYPE_OUTDIGIT2_WC : _bindgen_ty_1 = 53 ; pub const _NL_CTYPE_OUTDIGIT3_WC : _bindgen_ty_1 = 54 ; pub const _NL_CTYPE_OUTDIGIT4_WC : _bindgen_ty_1 = 55 ; pub const _NL_CTYPE_OUTDIGIT5_WC : _bindgen_ty_1 = 56 ; pub const _NL_CTYPE_OUTDIGIT6_WC : _bindgen_ty_1 = 57 ; pub const _NL_CTYPE_OUTDIGIT7_WC : _bindgen_ty_1 = 58 ; pub const _NL_CTYPE_OUTDIGIT8_WC : _bindgen_ty_1 = 59 ; pub const _NL_CTYPE_OUTDIGIT9_WC : _bindgen_ty_1 = 60 ; pub const _NL_CTYPE_TRANSLIT_TAB_SIZE : _bindgen_ty_1 = 61 ; pub const _NL_CTYPE_TRANSLIT_FROM_IDX : _bindgen_ty_1 = 62 ; pub const _NL_CTYPE_TRANSLIT_FROM_TBL : _bindgen_ty_1 = 63 ; pub const _NL_CTYPE_TRANSLIT_TO_IDX : _bindgen_ty_1 = 64 ; pub const _NL_CTYPE_TRANSLIT_TO_TBL : _bindgen_ty_1 = 65 ; pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN : _bindgen_ty_1 = 66 ; pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING : _bindgen_ty_1 = 67 ; pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN : _bindgen_ty_1 = 68 ; pub const _NL_CTYPE_TRANSLIT_IGNORE : _bindgen_ty_1 = 69 ; pub const _NL_CTYPE_MAP_TO_NONASCII : _bindgen_ty_1 = 70 ; pub const _NL_CTYPE_NONASCII_CASE : _bindgen_ty_1 = 71 ; pub const _NL_CTYPE_EXTRA_MAP_1 : _bindgen_ty_1 = 72 ; pub const _NL_CTYPE_EXTRA_MAP_2 : _bindgen_ty_1 = 73 ; pub const _NL_CTYPE_EXTRA_MAP_3 : _bindgen_ty_1 = 74 ; pub const _NL_CTYPE_EXTRA_MAP_4 : _bindgen_ty_1 = 75 ; pub const _NL_CTYPE_EXTRA_MAP_5 : _bindgen_ty_1 = 76 ; pub const _NL_CTYPE_EXTRA_MAP_6 : _bindgen_ty_1 = 77 ; pub const _NL_CTYPE_EXTRA_MAP_7 : _bindgen_ty_1 = 78 ; pub const _NL_CTYPE_EXTRA_MAP_8 : _bindgen_ty_1 = 79 ; pub const _NL_CTYPE_EXTRA_MAP_9 : _bindgen_ty_1 = 80 ; pub const _NL_CTYPE_EXTRA_MAP_10 : _bindgen_ty_1 = 81 ; pub const _NL_CTYPE_EXTRA_MAP_11 : _bindgen_ty_1 = 82 ; pub const _NL_CTYPE_EXTRA_MAP_12 : _bindgen_ty_1 = 83 ; pub const _NL_CTYPE_EXTRA_MAP_13 : _bindgen_ty_1 = 84 ; pub const _NL_CTYPE_EXTRA_MAP_14 : _bindgen_ty_1 = 85 ; pub const _NL_NUM_LC_CTYPE : _bindgen_ty_1 = 86 ; pub const __INT_CURR_SYMBOL : _bindgen_ty_1 = 262144 ; pub const __CURRENCY_SYMBOL : _bindgen_ty_1 = 262145 ; pub const __MON_DECIMAL_POINT : _bindgen_ty_1 = 262146 ; pub const __MON_THOUSANDS_SEP : _bindgen_ty_1 = 262147 ; pub const __MON_GROUPING : _bindgen_ty_1 = 262148 ; pub const __POSITIVE_SIGN : _bindgen_ty_1 = 262149 ; pub const __NEGATIVE_SIGN : _bindgen_ty_1 = 262150 ; pub const __INT_FRAC_DIGITS : _bindgen_ty_1 = 262151 ; pub const __FRAC_DIGITS : _bindgen_ty_1 = 262152 ; pub const __P_CS_PRECEDES : _bindgen_ty_1 = 262153 ; pub const __P_SEP_BY_SPACE : _bindgen_ty_1 = 262154 ; pub const __N_CS_PRECEDES : _bindgen_ty_1 = 262155 ; pub const __N_SEP_BY_SPACE : _bindgen_ty_1 = 262156 ; pub const __P_SIGN_POSN : _bindgen_ty_1 = 262157 ; pub const __N_SIGN_POSN : _bindgen_ty_1 = 262158 ; pub const _NL_MONETARY_CRNCYSTR : _bindgen_ty_1 = 262159 ; pub const __INT_P_CS_PRECEDES : _bindgen_ty_1 = 262160 ; pub const __INT_P_SEP_BY_SPACE : _bindgen_ty_1 = 262161 ; pub const __INT_N_CS_PRECEDES : _bindgen_ty_1 = 262162 ; pub const __INT_N_SEP_BY_SPACE : _bindgen_ty_1 = 262163 ; pub const __INT_P_SIGN_POSN : _bindgen_ty_1 = 262164 ; pub const __INT_N_SIGN_POSN : _bindgen_ty_1 = 262165 ; pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL : _bindgen_ty_1 = 262166 ; pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL : _bindgen_ty_1 = 262167 ; pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS : _bindgen_ty_1 = 262168 ; pub const _NL_MONETARY_DUO_FRAC_DIGITS : _bindgen_ty_1 = 262169 ; pub const _NL_MONETARY_DUO_P_CS_PRECEDES : _bindgen_ty_1 = 262170 ; pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE : _bindgen_ty_1 = 262171 ; pub const _NL_MONETARY_DUO_N_CS_PRECEDES : _bindgen_ty_1 = 262172 ; pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE : _bindgen_ty_1 = 262173 ; pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES : _bindgen_ty_1 = 262174 ; pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE : _bindgen_ty_1 = 262175 ; pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES : _bindgen_ty_1 = 262176 ; pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE : _bindgen_ty_1 = 262177 ; pub const _NL_MONETARY_DUO_P_SIGN_POSN : _bindgen_ty_1 = 262178 ; pub const _NL_MONETARY_DUO_N_SIGN_POSN : _bindgen_ty_1 = 262179 ; pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN : _bindgen_ty_1 = 262180 ; pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN : _bindgen_ty_1 = 262181 ; pub const _NL_MONETARY_UNO_VALID_FROM : _bindgen_ty_1 = 262182 ; pub const _NL_MONETARY_UNO_VALID_TO : _bindgen_ty_1 = 262183 ; pub const _NL_MONETARY_DUO_VALID_FROM : _bindgen_ty_1 = 262184 ; pub const _NL_MONETARY_DUO_VALID_TO : _bindgen_ty_1 = 262185 ; pub const _NL_MONETARY_CONVERSION_RATE : _bindgen_ty_1 = 262186 ; pub const _NL_MONETARY_DECIMAL_POINT_WC : _bindgen_ty_1 = 262187 ; pub const _NL_MONETARY_THOUSANDS_SEP_WC : _bindgen_ty_1 = 262188 ; pub const _NL_MONETARY_CODESET : _bindgen_ty_1 = 262189 ; pub const _NL_NUM_LC_MONETARY : _bindgen_ty_1 = 262190 ; pub const __DECIMAL_POINT : _bindgen_ty_1 = 65536 ; pub const RADIXCHAR : _bindgen_ty_1 = 65536 ; pub const __THOUSANDS_SEP : _bindgen_ty_1 = 65537 ; pub const THOUSEP : _bindgen_ty_1 = 65537 ; pub const __GROUPING : _bindgen_ty_1 = 65538 ; pub const _NL_NUMERIC_DECIMAL_POINT_WC : _bindgen_ty_1 = 65539 ; pub const _NL_NUMERIC_THOUSANDS_SEP_WC : _bindgen_ty_1 = 65540 ; pub const _NL_NUMERIC_CODESET : _bindgen_ty_1 = 65541 ; pub const _NL_NUM_LC_NUMERIC : _bindgen_ty_1 = 65542 ; pub const __YESEXPR : _bindgen_ty_1 = 327680 ; pub const __NOEXPR : _bindgen_ty_1 = 327681 ; pub const __YESSTR : _bindgen_ty_1 = 327682 ; pub const __NOSTR : _bindgen_ty_1 = 327683 ; pub const _NL_MESSAGES_CODESET : _bindgen_ty_1 = 327684 ; pub const _NL_NUM_LC_MESSAGES : _bindgen_ty_1 = 327685 ; pub const _NL_PAPER_HEIGHT : _bindgen_ty_1 = 458752 ; pub const _NL_PAPER_WIDTH : _bindgen_ty_1 = 458753 ; pub const _NL_PAPER_CODESET : _bindgen_ty_1 = 458754 ; pub const _NL_NUM_LC_PAPER : _bindgen_ty_1 = 458755 ; pub const _NL_NAME_NAME_FMT : _bindgen_ty_1 = 524288 ; pub const _NL_NAME_NAME_GEN : _bindgen_ty_1 = 524289 ; pub const _NL_NAME_NAME_MR : _bindgen_ty_1 = 524290 ; pub const _NL_NAME_NAME_MRS : _bindgen_ty_1 = 524291 ; pub const _NL_NAME_NAME_MISS : _bindgen_ty_1 = 524292 ; pub const _NL_NAME_NAME_MS : _bindgen_ty_1 = 524293 ; pub const _NL_NAME_CODESET : _bindgen_ty_1 = 524294 ; pub const _NL_NUM_LC_NAME : _bindgen_ty_1 = 524295 ; pub const _NL_ADDRESS_POSTAL_FMT : _bindgen_ty_1 = 589824 ; pub const _NL_ADDRESS_COUNTRY_NAME : _bindgen_ty_1 = 589825 ; pub const _NL_ADDRESS_COUNTRY_POST : _bindgen_ty_1 = 589826 ; pub const _NL_ADDRESS_COUNTRY_AB2 : _bindgen_ty_1 = 589827 ; pub const _NL_ADDRESS_COUNTRY_AB3 : _bindgen_ty_1 = 589828 ; pub const _NL_ADDRESS_COUNTRY_CAR : _bindgen_ty_1 = 589829 ; pub const _NL_ADDRESS_COUNTRY_NUM : _bindgen_ty_1 = 589830 ; pub const _NL_ADDRESS_COUNTRY_ISBN : _bindgen_ty_1 = 589831 ; pub const _NL_ADDRESS_LANG_NAME : _bindgen_ty_1 = 589832 ; pub const _NL_ADDRESS_LANG_AB : _bindgen_ty_1 = 589833 ; pub const _NL_ADDRESS_LANG_TERM : _bindgen_ty_1 = 589834 ; pub const _NL_ADDRESS_LANG_LIB : _bindgen_ty_1 = 589835 ; pub const _NL_ADDRESS_CODESET : _bindgen_ty_1 = 589836 ; pub const _NL_NUM_LC_ADDRESS : _bindgen_ty_1 = 589837 ; pub const _NL_TELEPHONE_TEL_INT_FMT : _bindgen_ty_1 = 655360 ; pub const _NL_TELEPHONE_TEL_DOM_FMT : _bindgen_ty_1 = 655361 ; pub const _NL_TELEPHONE_INT_SELECT : _bindgen_ty_1 = 655362 ; pub const _NL_TELEPHONE_INT_PREFIX : _bindgen_ty_1 = 655363 ; pub const _NL_TELEPHONE_CODESET : _bindgen_ty_1 = 655364 ; pub const _NL_NUM_LC_TELEPHONE : _bindgen_ty_1 = 655365 ; pub const _NL_MEASUREMENT_MEASUREMENT : _bindgen_ty_1 = 720896 ; pub const _NL_MEASUREMENT_CODESET : _bindgen_ty_1 = 720897 ; pub const _NL_NUM_LC_MEASUREMENT : _bindgen_ty_1 = 720898 ; pub const _NL_IDENTIFICATION_TITLE : _bindgen_ty_1 = 786432 ; pub const _NL_IDENTIFICATION_SOURCE : _bindgen_ty_1 = 786433 ; pub const _NL_IDENTIFICATION_ADDRESS : _bindgen_ty_1 = 786434 ; pub const _NL_IDENTIFICATION_CONTACT : _bindgen_ty_1 = 786435 ; pub const _NL_IDENTIFICATION_EMAIL : _bindgen_ty_1 = 786436 ; pub const _NL_IDENTIFICATION_TEL : _bindgen_ty_1 = 786437 ; pub const _NL_IDENTIFICATION_FAX : _bindgen_ty_1 = 786438 ; pub const _NL_IDENTIFICATION_LANGUAGE : _bindgen_ty_1 = 786439 ; pub const _NL_IDENTIFICATION_TERRITORY : _bindgen_ty_1 = 786440 ; pub const _NL_IDENTIFICATION_AUDIENCE : _bindgen_ty_1 = 786441 ; pub const _NL_IDENTIFICATION_APPLICATION : _bindgen_ty_1 = 786442 ; pub const _NL_IDENTIFICATION_ABBREVIATION : _bindgen_ty_1 = 786443 ; pub const _NL_IDENTIFICATION_REVISION : _bindgen_ty_1 = 786444 ; pub const _NL_IDENTIFICATION_DATE : _bindgen_ty_1 = 786445 ; pub const _NL_IDENTIFICATION_CATEGORY : _bindgen_ty_1 = 786446 ; pub const _NL_IDENTIFICATION_CODESET : _bindgen_ty_1 = 786447 ; pub const _NL_NUM_LC_IDENTIFICATION : _bindgen_ty_1 = 786448 ; pub const _NL_NUM : _bindgen_ty_1 = 786449 ; pub type _bindgen_ty_1 = u32 ; extern "C" { pub fn nl_langinfo ( __item : nl_item ) -> * mut :: std :: os :: raw :: c_char ; } # [ repr ( C ) ] # [ derive ( Debug , Copy , Clone ) ] pub struct __locale_struct { pub __locales : [ * mut __locale_data ; 13usize ] , pub __ctype_b : * const :: std :: os :: raw :: c_ushort , pub __ctype_tolower : * const :: std :: os :: raw :: c_int , pub __ctype_toupper : * const :: std :: os :: raw :: c_int , pub __names : [ * const :: std :: os :: raw :: c_char ; 13usize ] , } # [ test ] fn bindgen_test_layout___locale_struct ( ) { assert_eq ! ( :: std :: mem :: size_of :: < __locale_struct > ( ) , 232usize , concat ! ( "Size of: " , stringify ! ( __locale_struct ) ) ) ; assert_eq ! ( :: std :: mem :: align_of :: < __locale_struct > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( __locale_struct ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < __locale_struct > ( ) ) ) . __locales as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __locale_struct ) , "::" , stringify ! ( __locales ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < __locale_struct > ( ) ) ) . __ctype_b as * const _ as usize } , 104usize , concat ! ( "Offset of field: " , stringify ! ( __locale_struct ) , "::" , stringify ! ( __ctype_b ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < __locale_struct > ( ) ) ) . __ctype_tolower as * const _ as usize } , 112usize , concat ! ( "Offset of field: " , stringify ! ( __locale_struct ) , "::" , stringify ! ( __ctype_tolower ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < __locale_struct > ( ) ) ) . __ctype_toupper as * const _ as usize } , 120usize , concat ! ( "Offset of field: " , stringify ! ( __locale_struct ) , "::" , stringify ! ( __ctype_toupper ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: std :: ptr :: null :: < __locale_struct > ( ) ) ) . __names as * const _ as usize } , 128usize , concat ! ( "Offset of field: " , stringify ! ( __locale_struct ) , "::" , stringify ! ( __names ) ) ) ; } pub type __locale_t = * mut __locale_struct ; pub type locale_t = __locale_t ; extern "C" { pub fn nl_langinfo_l ( __item : nl_item , __l : locale_t ) -> * mut :: std :: os :: raw :: c_char ; } # [ repr ( C ) ] # [ derive ( Debug , Copy , Clone ) ] pub struct __locale_data { pub _address : u8 , }