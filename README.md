# Rust LaTeX Table Parser

Of course this is a work-in-progress. I'll add features and fix bugs as I find them necessary for other projects.

# Installing

```
cargo build
```

# Running (Example Output)

Below, defs[0-9].tex have "newcommand" macro declarations that represent data values in the table.
"nyt10.tex" contains the "tabular" environment command that defines the table.

```
./target/debug/latex_table_parse ~/Projects/paper/defs[0-9].tex ~/Projects/paper/tab/nyt10.tex
```

Output (as of the first commit, subject to change): 

  ```
  toprule      multicolumn    multicolumn
  cmidrulelr  cmidrulelr  multicolumn  emph  kernpt  multicolumn  hspace  multicolumn    multicolumn    multicolumn    multicolumn    multicolumn
  emph    Mo  Un  W07  W19  Un  W07  W19  Un  W07  W19  Un  W07  W19  Un  W07  W19  Un  W07  W19
  midrulemulticolumn  emph
  emph  Str  0.2  8.2  raisebox  raisebox  8.8  raisebox  raisebox  12.1  raisebox  raisebox  <1%  raisebox  raisebox  <1%  raisebox  raisebox  <1%  raisebox  raisebox
  emph  Acrn  0.2  6.5  11.0  9.2  6.4  10.9  9.7  11.4  13.9  13.7  <1%  <1%  <1%  <1%  <1%  <1%  <1%  <1%  <1%
  emph  Word  3.3  8.7  12.6  12.3  8.7  12.5  11.8  12.8  14.3  14.3  2%  22%  19%  2%  23%  16%  16%  48%  47%
  emph  FN  2.6  7.8  11.6  11.1  7.9  11.0  10.4  12.3  14.1  13.8  <1%  11%  7%  <1%  10%  6%  8%  32%  28%
  emph  LN  2.9  8.2  12.4  11.7  8.3  12.2  11.4  12.7  14.8  14.6  <1%  11%  8%  <1%  12%  7%  6%  33%  30%
  emph  FILN  2.9  8.5  13.1  13.1  8.6  13.2  13.2  12.8  15.4  15.3  <1%  1%  1%  <1%  2%  2%  <1%  5%  5%
  emph  FNLN  3.4  8.8  13.7  13.3  9.2  13.8  12.9  12.8  15.9  15.3  <1%  <1%  <1%  <1%  <1%  <1%  <1%  <1%  <1%
  emph  Ctry  5.0  8.6  9.0  9.0  8.6  8.9  8.9  9.1  9.1  9.1  77%  94%  93%  75%  90%  89%  97%  98%  97%
  emph  Rgn  4.1  10.7  15.0  14.6  10.2  14.3  13.6  14.1  16.8  16.7  2%  10%  8%  1%  9%  7%  3%  15%  14%
  emph  Natl  3.8  8.2  8.8  8.8  8.0  8.8  8.7  8.9  8.9  8.9  66%  90%  91%  61%  90%  88%  97%  98%  98%
  multicolumn  emph
  emph  Str  3.0  7.6  10.0  9.6  7.5  9.6  9.2  10.5  10.8  10.7  37%  74%  65%  35%  67%  60%  74%  88%  11%
  emph  Acrn  2.0  6.5  8.7  7.7  6.4  8.1  7.8  9.1  9.5  9.5  44%  75%  59%  43%  67%  61%  81%  91%  90%
  emph  Word  3.2  8.1  10.9  10.6  7.9  10.6  10.2  11.2  11.8  11.7  29%  69%  63%  27%  64%  57%  74%  88%  87%
  emph  FN  2.9  6.9  8.8  8.5  7.0  8.5  8.3  9.2  9.4  9.4  53%  82%  76%  52%  76%  72%  89%  95%  94%
  emph  LN  2.9  7.4  9.6  9.1  7.4  9.2  8.9  9.9  10.4  10.3  47%  77%  70%  46%  71%  66%  84%  92%  91%
  emph  FILN  3.2  9.2  12.4  12.1  8.9  12.0  11.4  12.2  13.5  13.4  29%  60%  54%  27%  55%  49%  56%  75%  73%
  emph  FNLN  3.5  9.2  13.0  12.8  9.5  13.0  12.5  12.5  14.3  14.2  26%  56%  53%  27%  56%  51%  49%  71%  70%
  emph  Ctry  3.0  5.6  5.8  5.7  5.6  5.7  5.7  5.8  5.8  5.8  92%  99%  96%  93%  97%  96%  97%  98%  98%
  emph  Rgn  3.1  7.4  9.3  8.9  7.3  8.9  8.6  9.6  9.9  9.9  53%  81%  75%  51%  75%  71%  88%  94%  93%
  emph  Natl  2.5  5.2  5.4  5.4  5.1  5.3  5.3  5.4  5.4  5.4  95%  99%  99%  90%  99%  98%  100%  100%  100%
  ```

The original table looked like this:

```
\newcommand\nytxunknown{\raisebox{1.25pt}{---}}
\begin{tabular}{l@{\hspace{0.785em}}l@{}r@{\hspace{3pt}}*{3}{@{\hspace{6pt}}*{3}{@{\hspace{1pt}}r@{\hspace{1pt}}}}@{\hspace{12pt}}*{3}{@{\hspace{6pt}}*{3}{@{\hspace{1pt}}r@{\hspace{2pt}}}}}
\toprule
& &
& \multicolumn{9}{c}{Leaked information (bits)}
& \multicolumn{9}{c}{Probability correct guess} \\
\cmidrule(lr){5-11}\cmidrule(lr){14-20}
\multicolumn{2}{l}{\emph{\textbf{Distr}}} & {Courier}\kern-8pt
& \multicolumn{3}{c}{{\tfontname}\hspace*{10pt}}
& \multicolumn{3}{c}{{\kern-6pt\afontname}}
& \multicolumn{3}{c}{{\kern-16pt\bfontname}}
& \multicolumn{3}{c}{{\tfontname}}
& \multicolumn{3}{c}{{\afontname}}
& \multicolumn{3}{c}{{\bfontname}}
\\
& \emph{Dict}
& Mo 
& {\nadjshortname} & {\wxiishortname} & {\wxxishortname}
& {\nadjshortname} & {\wxiishortname} & {\wxxishortname}
& {\nadjshortname} & {\wxiishortname} & {\wxxishortname}
& {\nadjshortname} & {\wxiishortname} & {\wxxishortname}
& {\nadjshortname} & {\wxiishortname} & {\wxxishortname}
& {\nadjshortname} & {\wxiishortname} & {\wxxishortname}
\\
\midrule
\multicolumn{6}{l}{\emph{\textbf{\uniname}}}
\\
& \emph{\strname}
& \hyunytstrnadjrx
& \hyunytstrnadjtx & \nytxunknown & \nytxunknown
& \hyunytstrnadjax & \nytxunknown & \nytxunknown
& \hyunytstrnadjbx & \nytxunknown & \nytxunknown
& \pgupctnytstrnadjtx & \nytxunknown & \nytxunknown
& \pgupctnytstrnadjax & \nytxunknown & \nytxunknown
& \pgupctnytstrnadjbx & \nytxunknown & \nytxunknown

[ ... many more row definitions ... ]

\\
& \emph{\natlname}
& \hywavgnytnatlnadjrx
& \hywavgnytnatlnadjtx & \hywavgnytnatlwxiitx & \hywavgnytnatlwxxitx
& \hywavgnytnatlnadjax & \hywavgnytnatlwxiiax & \hywavgnytnatlwxxiax
& \hywavgnytnatlnadjbx & \hywavgnytnatlwxiibx & \hywavgnytnatlwxxibx
& \pgwavgpctnytnatlnadjtx & \pgwavgpctnytnatlwxiitx & \pgwavgpctnytnatlwxxitx
& \pgwavgpctnytnatlnadjax & \pgwavgpctnytnatlwxiiax & \pgwavgpctnytnatlwxxiax
& \pgwavgpctnytnatlnadjbx & \pgwavgpctnytnatlwxiibx & \pgwavgpctnytnatlwxxibx
\\
\bottomrule
\end{tabular}
```
