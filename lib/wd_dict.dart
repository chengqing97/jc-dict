import 'dart:io';
import 'package:http/http.dart' as httpClient;

enum Status { success, noResult }

class Suggestion {
  String word;
  String? def;
  Suggestion(this.word, this.def);
}

class Result {
  String? ukPronunciation;
  String? usPronunciation;
  String? definition;
  Status status;
  List<Suggestion>? suggestions;

  Result({this.ukPronunciation, this.usPronunciation, this.definition, this.status = Status.success, this.suggestions});
}

Future<Result> lookup(String toSearch) async {
  final url = Uri.parse("https://dict.youdao.com/w/${toSearch}");
  var response = await httpClient.read(url);

  final isTranslate = RegExp(r'<div id="fanyiToggle">').hasMatch(response);
  if (isTranslate) return grabTranslation(response);

  final hasResult = RegExp(r'<h2 class="wordbook-js">').hasMatch(response) && !RegExp(r'您要找的是不是').hasMatch(response);
  if (!hasResult) return grabSuggestion(response);

  final isChinese = RegExp(r'英语怎么说').hasMatch(response);
  if (isChinese) return grabEnglishWords(response);

  return grabFormalDefinition(response);
}

// Translate
Result grabTranslation(String response) {
  final definitionPart = RegExp(r'(?<=<div class="trans-container">)[\s\S]*?(?=</div>)').firstMatch(response)?.group(0);

  if (definitionPart == null) return Result(status: Status.noResult);

  final definition = RegExp(r'(?<=<p>).*(?=</p>)').allMatches(definitionPart).toList()[1].group(0);
  return Result(definition: definition);
}

// Chinese to English
Result grabEnglishWords(String response) {
  final definitionPart = RegExp(r'(?<=<div class="trans-container">)[\s\S]*?(?=</div>)').firstMatch(response)?.group(0);
  final definitionsRaw = RegExp(r'(?<=<p class="wordGroup">)[\s\S]*?(?=</p>)').allMatches(definitionPart ?? "");
  var definitionLines = <String>[];
  for (var item in definitionsRaw) {
    final wordType = RegExp(r'(?<=;">)[\s\S]*?(?=</span>)').firstMatch(item.group(0) ?? "")?.group(0);
    final def = RegExp(r'(?<=E2Ctranslation">)[\s\S]*?(?=</a>)').firstMatch(item.group(0) ?? "")?.group(0);
    var definitionLine = "";
    if (wordType != null) definitionLine += "$wordType ";
    definitionLine += "$def";
    definitionLines.add(definitionLine);
  }
  return Result(definition: definitionLines.join('\n'));
}

// English to Chinese
Result grabFormalDefinition(String response) {
  final definitionPart =
      RegExp(r'(?<=<div class="trans-container">\s*<ul>)[\s\S]*?(?=</ul>)').firstMatch(response)?.group(0);
  final definitionsRaw = RegExp(r'(?<=<li>)[\s\S]*?(?=</li>)').allMatches(definitionPart ?? "");
  final definitions = definitionsRaw.map((item) => item.group(0) ?? "").join("\n");

  final ukPronunciation = RegExp(r'(?<=<span class="pronounce">英[\s\S]*<span class="phonetic">)[\s\S]*?(?=</span>)')
      .firstMatch(response)
      ?.group(0);
  final usPronunciation = RegExp(r'(?<=<span class="pronounce">美[\s\S]*<span class="phonetic">)[\s\S]*?(?=</span>)')
      .firstMatch(response)
      ?.group(0);

  return Result(definition: definitions, ukPronunciation: ukPronunciation, usPronunciation: usPronunciation);
}

// Suggestion
Result grabSuggestion(String response) {
  final suggestionsRaw = RegExp(r'(?<=<p class="typo-rel">)[\s\S]*?(?=</p>)').allMatches(response);
  if (suggestionsRaw.isEmpty) return Result(status: Status.noResult);
  var suggestions = <Suggestion>[];

  for (var item in suggestionsRaw) {
    final word = RegExp(r'(?<=class="search-js">)[\s\S]*?(?=</a></span>)').firstMatch(item.group(0) ?? "")?.group(0);
    final def = RegExp(r'(?<=</span>)[\s\S]*').firstMatch(item.group(0) ?? "")?.group(0)?.trim();

    if (word != null) suggestions.add(Suggestion(word, def));
  }
  return Result(suggestions: suggestions.isEmpty ? null : suggestions, status: Status.noResult);
}

enum Accent { uk, us }

Future<String?> getVoiceUrl(String keyword, Accent accent) async {
  final url = Uri.parse("https://www.merriam-webster.com/dictionary/${keyword}");
  var response = await httpClient.read(url);

  var mp3Path = RegExp(r'https://[\S]*.mp3').firstMatch(response)?.group(0);

  return mp3Path;
}

// Future<Uri?> getVoiceUrl(String keyword, Accent accent) async {
//   print("searching for $keyword");
//   final url = Uri.parse("https://dictionary.cambridge.org/dictionary/english/${keyword}");
//   var response = await httpClient.read(url);

//   print("response");
//   print(response);

//   RegExp regex;

//   switch (accent) {
//     case Accent.uk:
//       regex = RegExp(r'media/english/uk_pron/[\S]*.mp3');
//       break;
//     case Accent.us:
//       regex = RegExp(r'media/english/us_pron/[\S]*.mp3');
//       break;
//   }
//   var mp3Path = regex.firstMatch(response)?.group(0);

//   if (mp3Path == null) return null;

//   return Uri.parse("https://dictionary.cambridge.org/${mp3Path}");
// }
