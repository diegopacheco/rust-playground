class HelloWorld {
    private static native String hello(String input);

    static {
        System.loadLibrary("main");
    }

    public static void main(String[] args) {
        String output = HelloWorld.hello("Diego Pacheco");
        System.out.println(output);
    }
}
