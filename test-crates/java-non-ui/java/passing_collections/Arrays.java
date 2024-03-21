package passing_collections;

import java.util.List;

public class Arrays {

  public String test_with_string_array(String[] x) {
    StringBuilder result = new StringBuilder();
    for (String el: x) {
        result.append(el);
    }
    return result.toString();
  }

  public void test_with_string_list(List<String> x) {
    for (String el: x) {
        System.out.println(el);
    }
  }
}