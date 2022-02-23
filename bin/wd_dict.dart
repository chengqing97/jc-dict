import 'package:test/expect.dart';
import 'package:wd_dict/wd_dict.dart';
import 'package:args/args.dart';
import 'dart:io';
import 'package:colorize/colorize.dart';
import 'package:mp3_player/audio_player.dart';

const VERSION_NUMBER = "0.0.1";

Future process(String toSearch, [String? lastSearch]) async {
  try {
    if ((toSearch == "1" || toSearch == "2") && lastSearch != null) {
      var url = await getVoiceUrl(lastSearch, Accent.uk);
      print(url);
      var player = await Player.run();
      player.play(url);

      return;
    }
    final result = await lookup(toSearch);
    switch (result.status) {
      case Status.noResult:
        if (result.suggestions == null) {
          print(Colorize("No result").red());
        } else {
          print(("Are you looking for"));
          for (var item in result.suggestions!) {
            var suggestion = Colorize(item.word).lightMagenta().toString();
            if (item.def != null && item.def != "") suggestion += ": ${item.def}";
            print(suggestion);
          }
        }
        break;
      case Status.success:
        if (result.definition == null) {
          return print(Colorize("No result").red());
        }
        var pronunciation = "";
        if (result.ukPronunciation != null) pronunciation += "英 ${result.ukPronunciation}  ";
        if (result.usPronunciation != null) pronunciation += "美 ${result.usPronunciation}";
        if (pronunciation.isNotEmpty) print(Colorize(pronunciation).cyan());
        print(result.definition);
    }
  } catch (error) {
    stderr.writeln(error);
  }
}

void main(List<String> rawArgs) async {
  var argParser = ArgParser();

  argParser
    ..addFlag('help', abbr: 'h', negatable: false, help: "Show help message")
    ..addFlag('version', abbr: 'v', negatable: false, help: "Show version");

  ArgResults args;
  try {
    args = argParser.parse(rawArgs);
  } catch (e) {
    print("Invalid flag.");
    print("(Run 'wd --help' for more information)");
    exit(0);
  }

  if (args["help"]) {
    print('''
CLI 有道词典

${argParser.usage}

In interactive mode, you can input '1' for UK pronunciation and input '2' for US pronunciation of the previous searched word"
''');
  } else if (args["version"]) {
    print(VERSION_NUMBER);
  } else if (args.rest.isEmpty) {
    var history = <String>[];
    while (true) {
      stdout.write('~ ');
      final toSearch = stdin.readLineSync();
      if (toSearch == null || toSearch.isEmpty) continue;
      var lastSearch = history.isNotEmpty ? history.last : null;
      if (int.tryParse(toSearch) == null) history.add(toSearch);
      await process(toSearch, lastSearch);
      print("");
    }
  } else {
    final toSearch = args.rest.join(" ");
    await process(toSearch);
  }
}
