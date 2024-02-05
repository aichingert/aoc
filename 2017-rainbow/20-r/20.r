
get_input <- function() {
    chars <- strsplit(readLines("../input/20"), ", ")

    vecs = matrix(numeric(3), nrow = length(chars), ncol = 3)

    for (i in 1:length(chars)) {
        print(vecs[i])
        # print(chars[i])
    }

    return (vecs)
}

apply <- function(vec) {

}

vec = get_input()
