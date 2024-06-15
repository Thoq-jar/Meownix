CXX = g++

PKG_CFLAGS = $(shell pkg-config --cflags gtkmm-3.0)
PKG_LIBS = $(shell pkg-config --libs gtkmm-3.0)

OUTPUT = meowcode

SRCS = meowcode.cpp

OBJS = $(SRCS:.cpp=.o)

all: $(OUTPUT)

$(OUTPUT): $(OBJS)
	$(CXX) $(OBJS) -o $@ $(PKG_LIBS)

%.o: %.cpp
	$(CXX) $(CXXFLAGS) $(PKG_CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJS) $(OUTPUT)
